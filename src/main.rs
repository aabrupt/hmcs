use askama_axum::{IntoResponse as _, Template};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{debug_handler, Router};
use clap::{Parser, ValueEnum};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{query_as, SqlitePool};

pub mod error;

use error::Error;

#[derive(Parser)]
struct Cli {
    /// Set the program to run in persistent mode. Using a file instead of in memory database.
    #[arg(env = "SQLX_PERSISTENT", default_value_t = false)]
    persistent: bool,
    /// Database url
    #[arg(env = "DATABASE_URL", required_unless_present("persistent"))]
    database_url: String,
    /// Set the programs log level
    #[arg(env = "RUST_LOG", value_enum,  default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

#[derive(Clone, ValueEnum)]
enum LogLevel {
    Error,
    Info,
    Debug,
    Trace,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    tracing_subscriber::fmt::init();

    let database_url = if cli.persistent {
        cli.database_url.as_str()
    } else {
        ":memory:"
    };

    let database = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    if !cli.persistent {
        sqlx::migrate!().run(&database).await?;
    }

    let state = ApplicationState { database };

    let app = Router::new().route("/", get(index)).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, Clone)]
struct ApplicationState {
    database: SqlitePool,
}

#[debug_handler]
async fn index(State(state): State<ApplicationState>) -> Result<impl IntoResponse, Error> {
    let blog_posts = query_as!(
        Post,
        "SELECT author_tag, state, title, content, tags, description, keywords  FROM post
        INNER JOIN user     ON author_id = user.id
        INNER JOIN revision ON current_revision = revision.id
        WHERE state = 'published'"
    )
    .fetch_all(&state.database)
    .await?;

    Ok(Index {
        title: String::from("Home page"),
        blog_posts,
    }
    .into_response())
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    title: String,
    blog_posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "blog_post.html")]
struct Post {
    // Meta
    author_tag: String,
    state: PostState,

    // Content
    title: String,
    content: String,
    tags: String,

    // SEO
    description: String,
    keywords: String,
}

enum PostState {
    Draft,
    Published,
    Trashed,
}

/// WARNING: This trait impl is only designed to handle the post state from the sqlx managed
/// database.
impl From<String> for PostState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "draft" => Self::Draft,
            "published" => Self::Published,
            "trashed" => Self::Trashed,
            _ => unreachable!(),
        }
    }
}
