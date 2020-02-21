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
// type Task = (String, String, String, String, String, String);
#[derive(Debug,Serialize, Deserialize)]
struct Task { 
    id: String,
    status: Status,
    subject: String,
    created_on : String,
    modified_on: String,
    completed_on:String
}
#[derive(Deserialize, Serialize, Debug)]
enum Status{
    NotStarted,
    InProgress,
    Complete
}
fn main_func() -> Result<(), Box<dyn Error>> {
    let filename = "statusUpdate.csv";
    let mut data_reader = csv::Reader::from_path(filename).expect("could not read from file");
    let mut iter  = data_reader.deserialize();
    if let Some(res) = iter.next() {
        let task_item: Task = res?;
    }
    
    Ok(())
}

fn main() {
    if let Err(err) = main_func() {
        println!("ERROR: {}", err);
        process::exit(1);
    }
}
