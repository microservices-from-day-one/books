# script to create the database

CREATE DATABASE books_development;

-- \c books_development;

CREATE TABLE books (
  book_id     UUID PRIMARY KEY ,
  title       VARCHAR(100) NOT NULL,
  author      VARCHAR(100) NOT NULL,
  slug        VARCHAR(100),
  description VARCHAR(255),
  price       INT,
  pages       INT,
  full_price  INT,
  isbn        VARCHAR(15),
  weight      INT,
  height      INT,
  width       INT,
  depth       INT,
  cover_image VARCHAR(1000)
);

CREATE INDEX ON books (slug);

CREATE TABLE categories (
  category_id UUID PRIMARY KEY,
  name        VARCHAR(100) NOT NULL,
  description VARCHAR(255),
  slug        VARCHAR(100),
  image       VARCHAR(1000)
);

CREATE TABLE books_categories (
  book_id UUID NOT NULL,
  category_id UUID NOT NULL,
  PRIMARY KEY(book_id, category_id)
);

-- \d+ books;
-- \d+ categories;
-- \d+ books_categories;
