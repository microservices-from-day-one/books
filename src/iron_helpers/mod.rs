use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedBody;
use urlencoded::UrlEncodedQuery;

// Extracts a string parameter from the path
pub fn extract_param_from_path(req: &Request, parameter: &str) -> String {
    req.extensions.get::<Router>().unwrap().find(parameter).unwrap_or("").to_string()
}

pub fn extract_param_value_from_encoded_body(req: &mut Request, parameter: &str) -> Option<String> {
    let encoded_body = req.get_ref::<UrlEncodedBody>().unwrap();
    match encoded_body.get(parameter) {
        Some(n) => Some(n[0].to_string()),
        None => None
    }
}

pub fn extract_i32_param_value_from_encoded_body(req: &mut Request, parameter: &str) -> Option<i32> {
    let string_value = extract_param_value_from_encoded_body(req, parameter);
    match string_value {
        Some(n) => Some(n.parse::<i32>().unwrap()),
        None => None
    }
}

pub fn extract_string_param(req: &mut Request, parameter: &str) -> Option<String> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => {
            let param = hashmap.get(parameter);
            match param {
                Some(n) => Some(n[0].parse::<String>().unwrap()),
                None => None
            }
        },
        Err(_) => None
    }
}
