CREATE TABLE IF NOT EXISTS post (
  -- Meta
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  author_id INTEGER NOT NULL,
  state TEXT NOT NULL CHECK (state IN ('draft', 'published', 'trashed')),

  -- Content
  title TEXT NOT NULL CHECK (LENGTH(title) <= 25),
  current_revision INTEGER NOT NULL,
  tags TEXT NOT NULL,

  -- SEO
  description TEXT NOT NULL,
  keywords TEXT NOT NULL,

  FOREIGN KEY (author_id) REFERENCES user(id)
);

---

create table if not exists revision (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  last_revision INTEGER,
  content TEXT NOT NULL,
  FOREIGN KEY (last_revision) REFERENCES revisions(id)
);
