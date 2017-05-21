extern crate iron;
extern crate persistent;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate uuid;

use std::io::Read;
use std::env;
use iron::prelude::*;
use iron::status;
use persistent::Read as PRead;
use router::Router;
use uuid::Uuid;
use serde::*;
use serde_json::*;

#[macro_use]
mod db;
mod dal;

macro_rules! try_or_500 {
	($expr:expr) => (match $expr {
		Ok(val) => val,
		Err(e) => {
			println!("Errored: ${:?}", e);
			return Ok(Response::with((status::InternalServerError)))
		}
	})
}

macro_rules! try_or_400 {
	($expr:expr) => (match $expr {
		Ok(val) => val,
		Err(e) => {
			println!("Invalid Data: ${:?}", e);
			return Ok(Response::with((status::BadRequest)))
		}
	})
}

fn check_auth(conn : &db::PostgresConnection, app_req : &dal::ApplicationRequest) -> bool {
	dal::check_user(conn, app_req).unwrap()
}

fn create_auth(req : &mut Request) -> IronResult<Response> {
	let request = parse_request(req).unwrap();
	let conn = get_pg_connection!(req);
	match dal::create_auth(&conn, &request) {
		Ok(new_auth) => {
			let response_payload = try_or_500!(serde_json::to_string(&new_auth));
			Ok(Response::with((status::Ok, response_payload)))
		}
		Err(e) => {
			println!("Errored: ${:?}", e);
			Ok(Response::with((status::InternalServerError)))
		}
	}
}
fn get_sequence(req : &mut Request) -> IronResult<Response> {
	let conn = get_pg_connection!(req);
	let mut request = dal::ApplicationRequest::new();
	let auth_key = req.extensions.get::<Router>().unwrap().find("auth_key").unwrap_or("none");
	let sequence = req.extensions.get::<Router>().unwrap().find("sequence_key").unwrap_or("none");
	request.auth.auth_key = Uuid::parse_str(auth_key).unwrap();
	if !check_auth(&conn, &request) {
		return Ok(Response::with(status::BadRequest));
	}
	match dal::get_sequence(&conn, &request) {
		Ok(sequence) => {
			let response_payload = try_or_400!(serde_json::to_string(&sequence));
			Ok(Response::with((status::Ok, response_payload)))
		},
		Err(e) => {
			println!("Errored: ${:>}", e);
			Ok(Response::with((status::InternalServerError)))
		}
	}
}

fn get_sequences(req : &mut Request) -> IronResult<Response> {

    let conn = get_pg_connection!(req);
    let mut request = dal::ApplicationRequest::new();
    let auth_key = req.extensions.get::<Router>().unwrap().find("auth_key").unwrap_or("none");
    request.auth.auth_key = Uuid::parse_str(auth_key).unwrap();

	if !check_auth(&conn, &request) {
		return Ok(Response::with((status::BadRequest)));
	}
	match dal::list_sequences(&conn, &request) {
		Ok(sequences) => {
			let response_payload = try_or_400!(serde_json::to_string(&sequences));
			Ok(Response::with((status::Ok, response_payload)))
		},
		Err(e) => {
			println!("Errored: ${:?}", e);
			Ok(Response::with((status::InternalServerError)))
		}
	}
}

fn create_sequence(req: &mut Request) -> IronResult<Response> {
	let request = parse_request(req).unwrap();
	let conn = get_pg_connection!(req);
	if !check_auth(&conn, &request) {
		return Ok(Response::with((status::BadRequest)));
	}	
	match dal::create_sequence(&conn, &request) {
		Ok(sequence) => {
			let response_payload = try_or_400!(serde_json::to_string(&sequence));
			Ok(Response::with((status::Created, response_payload)))
		},
		Err(e) => {
			println!("Errored: {:?}", e);
			Ok(Response::with((status::InternalServerError))) 
		}
	}
}

fn parse_request(req: &mut Request) -> Result<dal::ApplicationRequest> {
	let mut payload = String::new();
	req.body.read_to_string(&mut payload);
	let app_request : dal::ApplicationRequest = serde_json::from_str(&payload)?;
	Ok(app_request)
}

fn main() {
	let mut router = Router::new();
	router.post("/auth", create_auth, "auth");
	router.get("/sequences/:auth_key", get_sequences, "sequences");
	router.get("/sequences/:auth_key/:sequence_key/current", create_sequence, "sequences/next");
    router.get("/sequences/:auth_key/:sequence_key/next", get_sequence, "sequences/current");
	
	let pool = db::get_pool(&db_conn());
	
	db::setup_database(pool.get().unwrap());
	let uri = listening_port();
	println!("Starting to listen on {}", uri);
	let mut chain = Chain::new(router);
	chain.link(PRead::<db::PostgresDB>::both(pool));
	Iron::new(chain).http(uri).unwrap();
}
fn listening_port() -> String {
    match env::var("LISTENING_PORT") {
        Ok(val) => val,
        Err(_) => "localhost:8080".to_string()
    }
}
fn db_conn() -> String {
	match env::var("DATABASE_URL") {
		Ok(val) => val,
		Err(_) => "postgres://Alex:mypass123@localhost:5432/myapp".to_string()
	}
}