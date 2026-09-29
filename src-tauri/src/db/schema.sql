PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = on;

CREATE TABLE IF NOT EXISTS projects(
    id INTEGER  PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL,
    last_opened INTEGER NOT NULL);

CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    relative_path TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT "",
    tags TEXT NOT NULL DEFAULT "[]",
    is_favorite INTEGER NOT NULL DEFAULT 0,
    word_count INTEGER NOT NULL DEFAULT 0,
    content TEXT NOT NULL DEFAULT "",
    created_at INTEGER NOT NULL DEFAULT 0,
    modified_at INTEGER NOT NULL DEFAULT 0,
    UNIQUE(project_id, relative_path)
    );


CREATE INDEX IF NOT EXISTS idx_files_project ON files(project_id);
CREATE INDEX IF NOT EXISTS idx_files_favorite ON files(
    project_id, is_favorite);

CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(
    title,
    content,
    content = "files",
    content_rowid = "id",
    tokenize = "unicode61"
);

CREATE TRIGGER IF NOT EXISTS files_ai AFTER INSERT ON files BEGIN 
    INSERT INTO files_fts(rowid,title,content) VALUES (new.id,new.title,new.content);
END;
CREATE TRIGGER IF NOT EXISTS files_ad AFTER DELETE ON files BEGIN
    INSERT INTO files_fts(files_fts,rowid,title,content)
VALUES("delete", old.id,old.title,old.content);
END;
CREATE TRIGGER IF NOT EXISTS files_au AFTER UPDATE ON files BEGIN
    INSERT INTO files_fts(files_fts, rowid, title, content)
VALUES("delete", old.id, old.title, old.content);
    INSERT INTO  files_fts(rowid, title, content) VALUES (new.id, new.title, new.content);
END;