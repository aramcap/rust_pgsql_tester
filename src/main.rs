// PostgreSQL connection tester, https://github.com/aramcap/rust_pgsql_tester, 2021

use postgres::{Client, NoTls, Error};
use std::env;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let url = if args.len() > 1 { &args[1]} else { "" };
    if url == "" {
        println!("PostgreSQL connection tester, https://github.com/aramcap/rust_pgsql_tester, 2021");
        println!("Execution example:");
        println!("./pgsql_tester postgresql://postgres:postgres@localhost:5432/postgres");
    } else {
        Client::connect(url, NoTls)?;
    }    
    Ok(())
}
