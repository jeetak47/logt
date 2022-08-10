use std::io::{self, BufRead};
use colored::*;
use serde_json::{Result, Value};


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


fn main() {
    // Statements here are executed when the compiled binary is called
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let sline =line.as_ref().unwrap();
        
       // println!(" \x1b[1m Available capacity: \x1B[33m{}\x1b[0m",sline);
      let result=  print_log(sline);
      if !result.is_ok() {
        println!("{}", sline);
      }
      
         
    }
   
}
