CREATE TABLE IF NOT EXISTS roms (
   name         TEXT PRIMARY KEY,
   title        TEXT NOT NULL,
   genre        TEXT NOT NULL,
   year         TEXT,
   manufacturer TEXT,
   params       TEXT,
   play_count   INTEGER NOT NULL DEFAULT 0,
   favourite    BOOLEAN NOT NULL DEFAULT FALSE,
   clone_of     TEXT
);