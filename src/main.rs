use std::io::{self, BufRead};
use colored::*;
use serde_json::{Result, Value};
use clap::Parser;
use std::path::{PathBuf};
use chrono::{Datelike, Timelike, Utc};
use std::fs::File;
use std::io::{Write};
fn print_log(data: &String) -> Result<()> {
    let v: Value = serde_json::from_str(&data)?;
    let level = color_level(if !v["level"].is_null() {v["level"].as_str().unwrap()} else{""});
    let timestamp =  if !v["timestamp"].is_null() {v["timestamp"].as_str().unwrap()} else{""};
    let logger = if !v["logger"].is_null() {v["logger"].as_str().unwrap()} else{""};
    let message = if !v["message"].is_null() {v["message"].as_str().unwrap()} else{""};
    let stack = if !v["stack"].is_null() {v["stack"].as_str().unwrap().red()} else{"".normal()};
    
    // Access parts of the data by indexing with square brackets.
    println!("{} {} {} {}:{} ", timestamp,level,logger,message,stack);
    Ok(())
}

fn color_level(level:&str)->ColoredString{    
    return match level {
        "TRACE" => level.green(),
        "INFO" => level.blue(),
        "DEBUG" => level.green(),        
        "WARN" => level.yellow(),
        "ERROR" => level.red(),
        _ => level.normal()
    };
}
#[derive(Parser)]
struct  Cli{
    /// Write output to FILE.
    #[clap(long, value_name = "FILE", value_parser)]
    file: Option<PathBuf>,
    /// Write  output to file with auto generated file name based on time. 
    #[clap(short='f')]
    auto_file: bool,
}

fn main() {
    // Statements here are executed when the compiled binary is called
      let args = Cli::parse();
      let mut file_path: Option<String> = None;
      if args.auto_file{
        let now = Utc::now();
        file_path=Option::Some(format!("{}{}{:01}{:02}",now.month(),now.day(),now.minute(),now.hour()));
        //println!("Auto file name {}",file_path.unwrap());
        //file_name=
      }else if !args.file.is_none(){
       file_path=Option::Some(args.file.unwrap().display().to_string());
        //println!("File name {}",file_path.unwrap())
      }


    let mut output:File;
    let stdin = io::stdin();
    if !file_path.is_none() {
    let mut output = File::create(file_path.unwrap()).expect("Error while creating file");
    }
    for line in stdin.lock().lines() {
        let mut sline =line.as_ref().unwrap();
        
       // println!(" \x1b[1m Available capacity: \x1B[33m{}\x1b[0m",sline);
      let result=  print_log(sline);
      if !result.is_ok() {
        println!("{}", sline);
        write!(output, sline);
      }
      
         
    }
   
}
