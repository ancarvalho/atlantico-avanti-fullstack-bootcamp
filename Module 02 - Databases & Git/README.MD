## First Challenge - DATABASE


### Suppose that you are working with a database of a library. The database has two principal tables Books & Authors. Each book has a association with their respective author a foreign key

Basic Structure:

* Authors:
  - id (primary key)
  - name (author name)
  - nationality

* Books:
  - id (primary key)
  - title (book title)
  - publication_year
  - author_id (foreign key with reference to authors table)