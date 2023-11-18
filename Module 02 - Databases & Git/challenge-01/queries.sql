

-- write a query to list all books published in 2020

SELECT title 
FROM books
WHERE publication_year = 2020;


-- write a query to list all authors who have at least 2 books on the library

SELECT a.name name
FROM authors a 
LEFT JOIN books b ON b.author_id = a.id
GROUP BY a.id
HAVING COUNT(b.title) > 1;


-- write a query to find the author with most books on the library

SELECT a.name name
FROM authors a 
LEFT JOIN books b ON b.author_id = a.id
GROUP BY a.id
ORDER BY COUNT(b.title) DESC
LIMIT 1;


-- write a query to find the name of the author and the most recent books

WITH temporary_table AS (
  SELECT b.author_id author_id, MAX(b.publication_year) publication_year
	FROM books b
	GROUP BY b.author_id
)
SELECT a.name name, b.title
FROM authors a 
LEFT JOIN books b ON b.author_id = a.id
LEFT JOIN temporary_table tt ON tt.author_id = a.id
WHERE tt.author_id = b.author_id AND tt.publication_year = b.publication_year;


-- write a query to find how many books each author have write

SELECT a.name name, COUNT(b.title) book_count 
FROM authors a 
LEFT JOIN books b ON b.author_id = a.id
GROUP BY a.id
ORDER BY book_count DESC;


-- write a query to list all books and their respective authors alfabetically ordinated

SELECT b.title title, a.name
FROM authors a 
LEFT JOIN books b ON b.author_id = a.id
ORDER BY a.name ASC;

-- write a query to calculate the average of all books publication year

SELECT FLOOR(AVG(b.publication_year)) average
FROM books b;

-- write a query to find the title of the olddest book on the library 

SELECT b.title
FROM books b
ORDER BY b.publication_year ASC
LIMIT 1;

-- write a query to find the total count of books on the library

SELECT COUNT(*) total_books_count
from books;

-- write a query to find all authors who havent books at the library 

SELECT a.name name
FROM authors a 
LEFT JOIN books b ON b.author_id = a.id
GROUP BY a.id
HAVING COUNT(b.title) < 1;



