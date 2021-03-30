fn main() -> Result<(),  Box<dyn std::error::Error>> {
    env_logger();
    Ok(())
}

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn execute_query_error(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn env_logger() -> () {
    env_logger::init();

    execute_query("DROP TABLE students");

    let response = execute_query_error("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}
