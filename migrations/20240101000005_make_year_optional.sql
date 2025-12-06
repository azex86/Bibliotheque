CREATE TABLE books_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    year INTEGER,
    description TEXT,
    subtitle TEXT,
    volume_number INTEGER
);

INSERT INTO books_new (id, title, author, year, description, subtitle, volume_number)
SELECT id, title, author, year, description, subtitle, volume_number FROM books;

DROP TABLE books;
ALTER TABLE books_new RENAME TO books;
