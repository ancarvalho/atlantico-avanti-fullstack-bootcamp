
DROP TABLE IF EXIST events;
DROP TABLE IF EXIST categories;
DROP TABLE IF EXIST places;

CREATE TABLE IF NOT EXIST places (  
  id UUID PRIMARY KEY DEFAULT get_random_uuid(),
  name VARCHAR(120) NOT NULL,
  address VARCHAR(169) NOT NULL,
  neighbothood VARCHAR(69) NOT NULL,
  city VARCHAR(50) NOT NULL,
  state VARCHAR(50) NOT NULL,
  country VARCHAR(46) NOT NULL,

);

CREATE TABLE IF NOT EXIST categories (
  id UUID PRIMARY KEY DEFAULT get_random_uuid(), 
  name VARCHAR(120) NOT NULL,
);

CREATE TABLE IF NOT EXIST events ( 
  id UUID PRIMARY KEY DEFAULT get_random_uuid(),
  name VARCHAR(120) NOT NULL,
  description TEXT NOT NULL,
  data TIMESTAMP NOT NULL,
  category_id UUID NOT NULL REFERENCES categories(id),
  place_id UUID NOT NULL REFERENCES places(id),


)
