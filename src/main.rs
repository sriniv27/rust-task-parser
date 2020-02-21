#![allow(unused)]
use chrono::*;
use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

// The `record` string is a 6 element string where the elements are
// ordered as:

//     ID,Status,Subject,CreatedOn,ModifiedOn,CompletedOn
type Task = (String, String, String, String, String, String);

fn main_func() -> Result<(), Box<dyn Error>> {
    let filename = "statusUpdate.csv";
    let mut data_reader = csv::Reader::from_path(filename).expect("could not read from file");
    for item in data_reader.deserialize() {
        let task: Task = item?;
        println!("Task: {:?}", task);
    }
    Ok(())
}

fn main() {
    if let Err(err) = main_func() {
        println!("ERROR: {}", err);
        process::exit(1);
    }
}
