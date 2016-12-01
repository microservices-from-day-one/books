use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use jsonway;
use persistent::Read;
use postgres::rows::Row;
use rustc_serialize::json;
use serde_json::value::Value as serde_Value;
use uuid::Uuid;

use iron_helpers;
use jsonapi;

#[derive(Debug, RustcEncodable)]
pub struct Category {
    category_id: Uuid,
    name: String,
    description: String,
    slug: String,
    image: String,
}

impl Category {
    fn from_row(row: Row) -> Category {
        Category {
            category_id: row.get("category_id"),
            name: row.get("name"),
            description: row.get("description"),
            slug: row.get("slug"),
            image: row.get("image"),
        }
    }
}

impl jsonapi::JsonApiAble for Category {
    fn to_jsonapi(&self) -> serde_Value {
        let json = jsonway::object(|json| {
            json.set("type", "categories".to_string());
            json.set("id", self.category_id.to_string());

            json.object("attributes", |json| {
                json.set("name", &self.name);
                json.set("description", &self.description);
                json.set("slug", &self.slug);
                json.set("image", &self.image);
            });
        });
        json.unwrap()
    }
}

// Saves a category
pub fn create_category(req: &mut Request) -> IronResult<Response> {
    // read post parameters
    let name = iron_helpers::extract_param_value_from_encoded_body(req, "name");
    let description = iron_helpers::extract_param_value_from_encoded_body(req, "description").unwrap();
    let image = iron_helpers::extract_param_value_from_encoded_body(req, "image").unwrap();

    // TODO: validate all required attributes
    // TODO: generate error dynamically
    if name.is_none() {
        let payload = "{\"missing\": \"name\"}";
        return Ok(Response::with((status::UnprocessableEntity, payload)));
    }

    let unwrapped_name = name.unwrap();
    let slug = unwrapped_name.clone().to_lowercase().replace(" ", "-");

    // create store object
    let category = Category {
        category_id: Uuid::new_v4(),
        name: unwrapped_name,
        description: description,
        slug: slug,
        image: image
    };

    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    conn.execute("INSERT INTO categories (category_id, name, description, slug) VALUES ($1, $2, $3, $4)",
             &[&category.category_id, &category.name, &category.description, &category.slug]).unwrap();

    println!("created category {:?} ", category);

    let payload = json::encode(&category).unwrap();
    Ok(Response::with((status::Created, Header(ContentType::json()), payload)))
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();

    let mut categories: Vec<Category> = Vec::new();
    for row in &conn.query("SELECT category_id, name, description, slug, image FROM categories ORDER BY name", &[]).unwrap() {
        let category = Category::from_row(row);
        categories.push(category);
    }

    let response = jsonapi::json_collection_wrapped_in("data", categories);
    Ok(Response::with((status::Ok,
                       Header(jsonapi::jsonapi_content_type()),
                       response)))
}

pub fn featured(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<::AppDb>>().unwrap();
    let conn = pool.get().unwrap();

    let mut categories: Vec<Category> = Vec::new();
    for row in &conn.query("SELECT category_id, name, description, slug, image FROM categories ORDER BY random()", &[]).unwrap() {
        let category = Category::from_row(row);
        categories.push(category);
    }

    let response = jsonapi::json_collection_wrapped_in("data", categories);
    Ok(Response::with((status::Ok,
                       Header(jsonapi::jsonapi_content_type()),
                       response)))
}
