# Ciclo 02 - Atlântico Avanti FullStack - Jingle Bytes


### Created With
- TypeScript
- NodeJS
- Express
- Prisma
- Postgres

### How To Start
*needed to create a .env with database url*
  ```bash
  npm install
  npx prima migrate dev
  npm run dev
  ```

### Routes

***Category***

- /category - POST - create an category (name*)
- /category/all - GET - get all categories
- /category/unique/:category_id - GET - get unique category with category_id
- /category/:category_id  - PUT - update an category (name) with category_id*
- /category/:category_id - DELETE - delete an category by category_id*

***Place***

- /place - POST - create an place (name*, address*, neighborhood*, city*, state*, country*)
- /place/all - GET - get all places
- /place/unique/:place_id - GET - get unique place with place_id
- /place/:place_id - PATCH - update an place (name, address, neighborhood, city, state, country) with place_id*
- /place/:place_id - DELETE - delete an place by place_id*

***Event***

- /event - POST - create an Event (name*, description, date*, category_id*, place_id*)
- /event/find?search=String&categories=List<Uuid>&dates=List<DateTime>&places=List<Uuid> - GET - filter events (**Filter Params)
- /event/all - GET - get all events
- /event/unique/:event_id - GET - get unique event with event_id
- /event/:event_id - PATCH - update an Event ( name, description, date, category_id, place_id) with event_id*
- /event/:event_id - DELETE - delete an Event by event_id*

#### Disclaimer

- all items marked with * are required

****Filter Params**:
- search String
- categories (Uuid list separated by comma)
- dates(list of 2 datas separated by comma MM/DD/YYYY or YYYY/DD/MM)
- places (Uuid list separated by comma)