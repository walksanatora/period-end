
#[path="src/structs.rs"]
mod structs;
use structs::*;

use schemars::schema_for;

fn main() {
    println!("starting build.rs for pend");
    println!("cargo:rustc-cfg=noimpl");
    println!("cargo:rerun-if-changed=sched.json");
    let schema = schema_for!(Week);
    println!("Here is the output format");
    std::fs::write("schema.json", serde_json::to_string(&schema).unwrap()).unwrap();
    println!("Finished Build.rs for pend");
}