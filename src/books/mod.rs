use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use jsonway;
use persistent::Read;
use postgres::rows::Rows;
use postgres::rows::Row;
use rustc_serialize::json;
use serde_json::value::Value as serde_Value;
use uuid::Uuid;

use iron_helpers;
use jsonapi;

#[derive(Debug, RustcEncodable)]
pub struct Book {
    book_id: Uuid,
    title: String,
    author: String,
    slug: String,
    description: String,
    isbn: String,
    price: i32,
    pages: i32,
    cover_image: String,
}

impl Book {
    fn from_row(row: Row) -> Book {
        Book {
            book_id: row.get("book_id"),
            title: row.get("title"),
            author: row.get("author"),
            slug: row.get("slug"),
            description: row.get("description"),
            isbn: row.get("isbn"),
            price: row.get("price"),
            pages: row.get("pages"),
            cover_image: row.get("cover_image"),
        }
    }
}

impl jsonapi::JsonApiAble for Book {
    fn to_jsonapi(&self) -> serde_Value {
        let json = jsonway::object(|json| {
            json.set("type", "books".to_string());
            json.set("id", self.book_id.to_string());

            json.object("attributes", |json| {
                json.set("title", &self.title);
                json.set("author", &self.author);
                json.set("slug", &self.slug);
                json.set("description", &self.description);
                json.set("isbn", &self.isbn);
                json.set("price", &self.price);
                json.set("pages", &self.pages);
                json.set("cover_image", &self.cover_image);
            });
        });
        json.unwrap()
    }
}

// Saves a book
pub fn create(req: &mut Request) -> IronResult<Response> {
    // read post parameters
    let title = iron_helpers::extract_param_value_from_encoded_body(req, "title");
    let author = iron_helpers::extract_param_value_from_encoded_body(req, "author");
    let description = iron_helpers::extract_param_value_from_encoded_body(req, "description").unwrap();
    let isbn = iron_helpers::extract_param_value_from_encoded_body(req, "isbn").unwrap();
    let price = iron_helpers::extract_i32_param_value_from_encoded_body(req, "price");
    let pages = iron_helpers::extract_i32_param_value_from_encoded_body(req, "pages");
    let cover_image = iron_helpers::extract_param_value_from_encoded_body(req, "cover_image");

    // TODO: validate all required attributes
    // TODO: generate error dynamically
    if title.is_none() {
        let payload = "{\"missing\": \"title\"}";
        return Ok(Response::with((status::UnprocessableEntity, payload)));
    }
    if price.is_none() {
        let payload = "{\"missing\": \"price\"}";
        return Ok(Response::with((status::UnprocessableEntity, payload)));
    }

    let unwrapped_title = title.unwrap();
    let slug = unwrapped_title.clone().to_lowercase().replace(" ", "-").replace(":", "");

    let book = Book {
        book_id: Uuid::new_v4(),
        title: unwrapped_title,
        author: author.unwrap(),
        description: description,
        isbn: isbn,
        price: price.unwrap(),
        pages: pages.unwrap(),
        slug: slug,
        cover_image: cover_image.unwrap(),
    };

    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    conn.execute("INSERT INTO books (book_id, title, description, isbn, \
        price, , pages, slug) VALUES ($1, $2, $3, $4, $5, $6, $7)",
             &[&book.book_id, &book.title, &book.description, &book.isbn, &book.price, &book.pages,
                &book.slug]).unwrap();

    println!("created book {:?} ", book);

    let payload = json::encode(&book).unwrap();
    Ok(Response::with((status::Created, Header(ContentType::json()), payload)))
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();

    let slug = iron_helpers::extract_string_param(req, "filter[slug]");

    let mut sql = String::new();
    sql.push_str("SELECT book_id, title, author, description, isbn, price, pages, \
        slug, cover_image FROM books");
    if slug.is_some() {
        sql.push_str(" WHERE slug='");
        sql.push_str(&slug.unwrap());
        sql.push_str("'");
    } else {
        sql.push_str(" LIMIT 10");
    }

    let mut books: Vec<Book> = Vec::new();
    for row in &conn.query(&sql, &[]).unwrap() {
        let book = Book::from_row(row);
        books.push(book);
    }

    let response = jsonapi::json_collection_wrapped_in("data", books);
    Ok(Response::with((status::Ok,
                       Header(jsonapi::jsonapi_content_type()),
                       response)))
}

pub fn related(req: &mut Request) -> IronResult<Response> {
    let book_id = iron_helpers::extract_param_from_path(req, "book_id");
    let book_uuid = Uuid::parse_str(&book_id).unwrap();

    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();

    let sql = "SELECT book_id, title, author, description, isbn, price, pages, \
        slug, cover_image FROM books WHERE book_id != $1 ORDER BY random() LIMIT 5";

    let mut books: Vec<Book> = Vec::new();
    for row in &conn.query(&sql, &[&book_uuid]).unwrap() {
        let book = Book::from_row(row);
        books.push(book);
    }

    let response = jsonapi::json_collection_wrapped_in("data", books);
    Ok(Response::with((status::Ok,
                       Header(jsonapi::jsonapi_content_type()),
                       response)))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    let book_id = iron_helpers::extract_param_from_path(req, "book_id");
    let book_uuid = Uuid::parse_str(&book_id).unwrap();

    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    let stmt = conn.prepare("SELECT book_id, title, author, description, isbn, price, pages, \
        slug, cover_image FROM books WHERE book_id=$1").unwrap();

    let rows = stmt.query(&[&book_uuid]).unwrap();
    let book = extract_book(rows);

    if book.is_none() {
        Ok(Response::with((status::NotFound, Header(ContentType::json()))))
    } else {
        let response = jsonapi::json_wrapped_in("data", book.unwrap());
        Ok(Response::with((status::Ok,
                           Header(jsonapi::jsonapi_content_type()),
                           response)))
    }
}

fn extract_book(rows: Rows) -> Option<Book> {
    if rows.len() == 0 {
        return None;
    } else {
        let row = rows.iter().next().unwrap();
        let book = Book::from_row(row);
        return Some(book);
    }
}
