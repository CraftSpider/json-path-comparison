extern crate jsonpath_plus as jsonpath;
extern crate serde_json;

use std::io;
use std::env;
use std::process;
use std::panic;

fn main() {
    panic::set_hook(Box::new(|_| {
        println!("panic occurred");
    }));

    let query = env::args().nth(1).unwrap();

    let json = serde_json::from_reader(io::stdin()).unwrap();

    let path = jsonpath::JsonPath::compile(&query[..])
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(2);
        });

    let result = path
        .find(&json)
/*        .unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    })*/;

    serde_json::to_writer(io::stdout(), &result).unwrap();
}
