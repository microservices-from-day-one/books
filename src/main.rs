extern crate rustc_serialize;
extern crate serde_json;
extern crate jsonway;

extern crate iron;
extern crate persistent;
extern crate router;
extern crate urlencoded;
extern crate logger;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate uuid;
extern crate getopts;

use getopts::Options;
use iron::prelude::*;
use iron::typemap::Key;
use persistent::Read;
use router::Router;
use logger::Logger;
use std::env;

use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};

mod books;
mod categories;
mod jsonapi;
mod iron_helpers;

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

pub struct AppDb;
impl Key for AppDb { type Value = PostgresPool; }

fn main() {
    let conn_string:String = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "postgres://clovescarneiro@localhost/books_development".to_string()
        // Err(_) => "postgres://tim@localhost/books_development".to_string()
    };

    println!("connecting to postgres: {}", conn_string);
    let pool = setup_connection_pool(&conn_string, 5);

    let port_number = host_port_number(env::args().collect());
    println!("port_number {}", &port_number);

    let mut router = Router::new();

    // books
    router.post("/v1/books", books::create);
    router.get("/v1/books/:book_id", books::show);
    router.get("/v1/books/:book_id/related", books::related);
    router.get("/v1/books", books::index);

    // categories
    router.post("/v1/categories", categories::create_category);
    router.get("/v1/categories", categories::index);
    router.get("/v1/categories/featured", categories::featured);

    let host = format!("localhost:{}", &port_number);
    println!("Starting server at {}", &host);

    let mut chain = Chain::new(router);
    chain.link(Read::<AppDb>::both(pool));
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http(&*host).unwrap();
}

fn host_port_number(args: Vec<String>) -> String {
    let mut opts = Options::new();
    opts.optopt("p", "port", "server port", "PORT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    match matches.opt_str("p") {
        Some(n) => n,
        None => "5000".to_string()
    }
}

fn setup_connection_pool(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str, r2d2_postgres::SslMode::None).unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    ::r2d2::Pool::new(config, manager).unwrap()
}
