use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error with sqlite occured: {0}")]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an error has occurred while interacting with the database",
            )
                .into_response(),
            Error::Migration(migrate_error) => unreachable!(),
        }
    }
}
