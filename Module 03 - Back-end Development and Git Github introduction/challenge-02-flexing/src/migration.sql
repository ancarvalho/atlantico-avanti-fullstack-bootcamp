
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
  category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE ON UPDATE CASCADE,
  place_id UUID NOT NULL REFERENCES places(id) ON DELETE CASCADE ON UPDATE CASCADE,

)


-- CreateIndex
CREATE INDEX "events_name_description_id_category_id_place_id_idx" ON "events"("name", "description", "id", "category_id", "place_id");

-- CreateIndex
CREATE UNIQUE INDEX "events_name_date_category_id_place_id_key" ON "events"("name", "date", "category_id", "place_id");

