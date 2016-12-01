use iron::headers::ContentType;
use iron::mime::*;

use jsonway;
use serde_json;
use serde_json::value::Value as serde_Value;

pub trait JsonApiAble {
    fn to_jsonapi(&self) -> serde_Value;
}

pub fn json_wrapped_in<T: JsonApiAble>(root: &str, element: T) -> String {
    let json = jsonway::object(|json| {
        json.set(root, element.to_jsonapi());
    });

    serde_json::to_string(&json.unwrap()).unwrap()
}

pub fn json_collection_wrapped_in<T: JsonApiAble>(root: &str, elements: Vec<T>) -> String {
    let json = jsonway::object(|json| {
        json.array(root, |json| {
            for e in elements.iter() {
                json.push(e.to_jsonapi());
            }
        });
    });

    serde_json::to_string(&json.unwrap()).unwrap()
}

pub fn jsonapi_content_type() -> ContentType {
    let mime: Mime = "application/vnd.api+json".parse().unwrap();
    ContentType(mime)
}
