DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS authors;


CREATE TABLE IF NOT EXISTS authors (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- gen_random_uuid only available on Postgres 13+
  name VARCHAR(100) NOT NULL ,
 	nationality VARCHAR(60) NOT NULL
);


CREATE TABLE IF NOT EXISTS books (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid()	,
  title  VARCHAR(255) NOT NULL ,
 	publication_year INT NOT NULL ,
  author_id UUID  NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
  UNIQUE(title)
);
