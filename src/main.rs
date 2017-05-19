extern crate iron;
extern crate persistent;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate router;
extern crate rustc_serialize;
extern crate uuid;

use std::io::Read;
use iron::prelude::*;
use iron::status;
use persistent::Read as PRead;
use router::Router;
use rustc_serialize::json;

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
			let response_payload = try_or_500!(json::encode(&new_auth));
			Ok(Response::with((status::Ok, response_payload)))
		}
		Err(e) => {
			println!("Errored: ${:?}", e);
			Ok(Response::with((status::InternalServerError)))
		}
	}
}

fn get_sequences(req : &mut Request) -> IronResult<Response> {
	let request = parse_request(req).unwrap();
	let conn = get_pg_connection!(req);
	if !check_auth(&conn, &request) {
		return Ok(Response::with((status::BadRequest)));
	}
	match dal::list_sequences(&conn, &request) {
		Ok(sequences) => {
			let response_payload = try_or_400!(json::encode(&sequences));
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
			let response_payload = try_or_400!(json::encode(&sequence));
			Ok(Response::with((status::Created, response_payload)))
		},
		Err(e) => {
			println!("Errored: {:?}", e);
			Ok(Response::with((status::InternalServerError))) 
		}
	}
}

fn parse_request(req: &mut Request) -> Result<dal::ApplicationRequest, json::DecoderError> {
	let mut payload = String::new();
	req.body.read_to_string(&mut payload);	
	let app_request : dal::ApplicationRequest = try!(json::decode(&payload));
	Ok(app_request)
}

fn main() {
	let mut router = Router::new();
	router.post("/auth", create_auth, "auth");
	router.get("/sequences", get_sequences, "sequences");
	router.post("/sequences/add", create_sequence, "sequences/add");
	
	let pool = db::get_pool("postgres://Alex:mypass123@localhost:5432/myapp");
	
	db::setup_database(pool.get().unwrap());
	let uri = "localhost:1337";
	println!("Starting to listen on {}", uri);
	let mut chain = Chain::new(router);
	chain.link(PRead::<db::PostgresDB>::both(pool));
	Iron::new(chain).http(uri).unwrap();
}	