CREATE TABLE book(
    id UUID NOT NULL PRIMARY KEY,
    name VARCHAR,
    author VARCHAR,
    publisher VARCHAR,
    units_available INTEGER,
    deleted BOOLEAN
);