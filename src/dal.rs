use postgres::error::Error;
use db;
use uuid::Uuid;

#[derive(RustcEncodable, RustcDecodable)]
pub struct ApplicationRequest {
	pub auth : UserAuthKey,
	pub sequence : Sequence
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Sequence {
	pub id: String,
	pub value: i64
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct UserAuthKey {
	pub auth_key : Uuid,
	pub email : String
}

pub fn log_use(conn : &db::PostgresConnection, req : &ApplicationRequest) -> Result<(), Error> {
	conn.execute("SELECT loguse($1::UUID, $2::VARCHAR);",
				&[&req.auth.auth_key, &req.sequence.id]).map(|_| ())
}

pub fn create_auth(conn : &db::PostgresConnection, req : &ApplicationRequest) -> Result<UserAuthKey, Error> {	
	let auth_email = &req.auth.email;
	for row in &conn.query("SELECT * FROM createnewauthid($1::VARCHAR)", &[&auth_email]).unwrap() {
		return Ok(UserAuthKey { auth_key: row.get(0), email: String::new() });
	}
	panic!("Was unable to create a new authorization key");
}
pub fn check_user(conn : &db::PostgresConnection, req : &ApplicationRequest) -> Result<bool, Error> {
	try!(log_use(conn, req));
	let qry = "SELECT active from user_auth_keys WHERE auth_key = $1";
	for row in &conn.query(qry, &[&req.auth.auth_key]).unwrap() {
		return Ok(row.get("active"));
	}
	Ok(false)
}
pub fn list_sequences(conn : &db::PostgresConnection, req : &ApplicationRequest) -> Result<Vec<Sequence>, Error> {
	try!(log_use(conn, req));
	let mut sequences: Vec<Sequence> = Vec::new();
	for row in &conn.query("SELECT sequence_id, sequence_value from sequences WHERE api_key = $1", &[&req.auth.auth_key]).unwrap() {
		sequences.push(Sequence {
			id: row.get(0),
			value: row.get(1)
		});
	}
	
	Ok(sequences)
}

pub fn create_sequence(conn: &db::PostgresConnection, req : &ApplicationRequest) -> Result<i64, Error> {
	try!(log_use(conn, req));
	for row in &conn.query("SELECT * FROM public.createnewsequence($1::varchar, $2::bigint, $3::uuid);", &[&req.sequence.id, &req.sequence.value, &req.auth.auth_key]).unwrap() {		
		return Ok(row.get(0));
	}
	panic!("Was unable to create a sequence");
}

