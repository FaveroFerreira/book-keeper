CREATE KEYSPACE IF NOT EXISTS book_keeper
    WITH REPLICATION = {
        'class': 'SimpleStrategy',
        'replication_factor': 1
        };

USE book_keeper;

CREATE TABLE IF NOT EXISTS check
(
    id UUID,
    PRIMARY KEY (id)
)
            WITH default_time_to_live = 1;

DROP TYPE IF EXISTS book_category;
DROP TABLE IF EXISTS student;
DROP TABLE IF EXISTS book;
DROP TABLE IF EXISTS borrow;

CREATE TYPE IF NOT EXISTS category (
    name TEXT,
    );

CREATE TABLE IF NOT EXISTS student
(
    id    UUID,
    name  TEXT,
    email TEXT,
    PRIMARY KEY ( id )
);

CREATE TABLE IF NOT EXISTS book
(
    id         UUID,
    title      TEXT,
    author     TEXT,
    publisher  TEXT,
    categories SET<FROZEN<category>>,
    PRIMARY KEY ( id )
);

CREATE TABLE IF NOT EXISTS borrow
(
    student_id   TEXT,
    book_id      TEXT,
    created_date TIMESTAMP,
    return_date  TIMESTAMP,
    PRIMARY KEY ( student_id, book_id )
);