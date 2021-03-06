mod state;
mod to_do;
mod processes;
use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use to_do::to_do_factory;
use processes::process_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> =
    read_file(String::from("./state.json"));
    let status: String;
    match &state.get(*&title) {
    Some(result) => {
    status = result.to_string().replace('\"', "");
    }
    None=> {
    status = String::from("pending");
    }
    }
    let item = to_do_factory(&status,
    title.to_string()).expect(&status);
    process_input(item, command.to_string(), &state);
   }