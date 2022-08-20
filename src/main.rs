use std::io::{self,Write,BufRead};
use colored::*;
use serde_json::{Value};
use clap::Parser;
use std::path::PathBuf;
use chrono::{Datelike, Timelike, Utc};
use std::fs::File;
fn print_log(data: String) -> Option<String> {
    let success= serde_json::from_str(&data);
    if success.is_ok(){
      let v: Value = success.unwrap(); 
    let level = color_level(if !v["level"].is_null() {v["level"].as_str().unwrap()} else{""});
    let timestamp =  if !v["timestamp"].is_null() {v["timestamp"].as_str().unwrap()} else{""};
    let logger = if !v["logger"].is_null() {v["logger"].as_str().unwrap()} else{""};
    let message = if !v["message"].is_null() {v["message"].as_str().unwrap()} else{""};
    let stack = if !v["stack"].is_null() {v["stack"].as_str().unwrap().red()} else{"".normal()};
    let d=format!("{} {} {} {}:{} ", timestamp,level,logger,message,stack);
    Some(d)
    }else {
    Some(data)
    }
    // Access parts of the data by indexing with square brackets.
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

    let mut output :Option<File>;
    let stdin = io::stdin();
    if !file_path.is_none() {
    output = Some(File::create(file_path.unwrap()).expect("Cant create file"));
    }else{
      output=None;
    }
    for line in stdin.lock().lines() {
      let sline =line.unwrap();
      let result=  print_log(sline);
      if result.is_some() {
        println!("{}", result.as_ref().unwrap());        
      if let Some(ref mut file) = output {
        writeln!(file,"{}", result.as_ref().unwrap()).expect("file not found");
    } 
    } 
    }
   
}
