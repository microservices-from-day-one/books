CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO CATEGORIES (category_id, name, description, slug, image)
VALUES (uuid_generate_v4(), 'Childrens Books', 'Something for the little ones.', 'children-s-books', 'http://sleepingshouldbeeasy.com/wp-content/uploads/2013/01/childrens-books-about-going-to-school-vertical.jpg');
INSERT INTO CATEGORIES (category_id, name, description, slug, image)
VALUES (uuid_generate_v4(), 'History', 'Learn about your past.', 'history', 'http://media1.s-nbcnews.com/ij.aspx?404;http://sys06-media.s-nbcnews.com:80/j/streams/2013/November/131120/2D9747505-Where_Were_You_book_jacket.blocks_desktop_vertical_tease.jpg');
INSERT INTO CATEGORIES (category_id, name, description, slug, image)
VALUES (uuid_generate_v4(), 'Romance', 'It is promise and hope. Titillations and excitations.', 'romance', 'https://s-media-cache-ak0.pinimg.com/564x/29/bd/9e/29bd9eafd49a185874e706fb4f896ba0.jpg');

INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), 'The Story of Diva and Flea', 'Mo Willems', 'Diva, a small yet brave dog, and Flea, a curious streetwise cat, develop an unexpected friendship in this unforgettable tale of discovery.', 987, '978-1484722848', 'the-story-of-diva-and-flea', 'https://images-na.ssl-images-amazon.com/images/I/61jurA6wsFL._SX258_BO1,204,203,200_.jpg', 80);
INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), '1491: New Revelations of the Americas Before Columbus', 'Charles C. Mann', 'In this groundbreaking work of science, history, and archaeology, Charles C. Mann radically alters our understanding of the Americas before the arrival of Columbus in 1492.', 1299, '978-1400032051', '1491-new-revelations', 'https://upload.wikimedia.org/wikipedia/en/b/b7/1491-cover.jpg', 541);
INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), 'Pride and Prejudice', 'Jane Austen', 'Pride and Prejudice is a novel of manners by Jane Austen, first published in 1813. The story follows the main character, Elizabeth Bennet, as she deals with issues of manners, upbringing, morality and education.', 951, '978-1484110980', 'pride-and-prejudice', 'https://themodernmanuscript.files.wordpress.com/2013/01/pride-and-prejudice-1946.jpg', 279);

INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), 'Pete the Cat: Scuba-Cat', 'James Dean ', 'Pete the Cat is going scuba diving! Before he hits the water, Captain Joe tells him about all the sea creatures he can encounter, and Pete is super excited to see a seahorse.', 319, '978-0062303882', 'pete-the-cat-scuba-cat', 'https://i.harperapps.com/covers/9780062303899/x300.png', 32);
INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), 'Pete the Cat: I Love My White Shoes', 'Eric Litwin', 'Pete the Cat goes walking down the street wearing his brand new white shoes. Along the way, his shoes change from white to red to blue to brown to WET as he steps in piles of strawberries, blueberries, and other big messes.', 1139, '978-0061906237', 'pete-the-cat-i-love-my-white-shoes', 'https://i.harperapps.com/covers/9780061906220/x300.png', 40);
INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), 'Pete the Cat and His Four Groovy Buttons', 'Eric Litwin', 'Pete the Cat is wearing his favorite shirt—the one with the four totally groovy buttons. But when one falls off, does Pete cry? Goodness, no! He just keeps on singing his song—after all, what could be groovier than three groovy buttons?', 1175, '978-0062110589', 'pete-the-cat-and-his-four-groovy-buttons', 'https://i.harperapps.com/covers/9780062110589/x300.png', 40);
INSERT INTO books (book_id, title, author, description, price, isbn, slug, cover_image, pages)
VALUES (uuid_generate_v4(), 'Pete the Cat: Five Little Pumpkins', 'James Dean', 'Pete the Cat takes on the classic favorite children''s song "Five Little Pumpkins" in New York Times bestselling author James Dean''s Pete the Cat: Five Little Pumpkins. Join Pete as he rocks out to this cool adaptation of the classic Halloween song!', 791, '978-0062304186', 'pete-the-cat-five-little-pumpkins', 'https://i.harperapps.com/covers/9780062304186/x300.png', 32);
