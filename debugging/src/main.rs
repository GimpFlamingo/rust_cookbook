#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(query: &str) {
	debug!("Executing query: {}", query);
}

fn execute_query_to_console(_query: &str) -> Result<(), &'static str> {
	Err("I'm afraid I can't do that")
}

fn main() {
	env_logger::init();

	execute_query("DROP TABLE students");
	let response = execute_query_to_console("DROP TABLE students");
	if let Err(err) = response {
		error!("Failed to execute query: {}", err);
	}
}
