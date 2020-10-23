use sled::*;
use std::{
    io::{
        self,
        Write,
    },
    process,
    str
};
use std::ops::Deref;

enum Command <'a>{
    HELP,
    EXIT,
    LIST,
    INSERT(&'a str, &'a str),
    GET(&'a str),
    UPDATE(&'a str, &'a str),
    DELETE(&'a str),
    ERROR
}

fn main() {
    let db : sled::Db = sled::open("dummy_db").unwrap();
    let separator = "-----------------------------";
    println!("{}", separator);
    println!("Welcome!");
    println!("{}", separator);
    println!("For help, please type HELP");
    let mut end = false;
    while !end {
        println!("{}", separator);
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match parse_input(&input) {
            Command::ERROR => println!("You entered an invalid command"),
            Command::HELP => show_help(),
            Command::EXIT => end = true,
            Command::LIST => list(&db),
            Command::INSERT(key, value) => insert(&db, key, value),
            Command::GET(key) => println!("{}", get(&db, key).trim()),
            Command::UPDATE(key, value) => update(&db, key, value),
            Command::DELETE(key) => delete(&db, key),
        }
    }
}

fn insert(db: &sled::Db, key: &str, value: &str){
    println!("Inserting {}: {}", key, value);
    db.insert(key, value);
}
fn get(db: &sled::Db, key: &str) -> String{
    println!("Getting {}", key);
    let res = db.get(key);
    let res = match res {
        Ok(b) => b,
        Err(_) => None,
    };
    let bytes = match res {
        Some(b) => b,
        None => IVec::from(""),
    };
    let s = str::from_utf8(bytes.deref()).unwrap();
    if s == "" {
        return String::from("Key '") + key + "' not found";
    }
    return s.to_owned();
}
fn update(db: &sled::Db, key: &str, value: &str){
    println!("Updating {}: {}", key, value);
    let old = get(&db, key);
    let res =
        db.compare_and_swap(&key, Some(old.trim()), Some(value)).unwrap();
    match res {
        Ok(_) => println!("Successful!"),
        Err(_) => println!("{} does not exist", key),
    }
}
fn delete(db: &sled::Db, key: &str){
    println!("Deleting {}", key);
    let res = db.remove(key);
    let res = match res {
        Ok(opt) => opt,
        Err(_) => None,
    };
    let res = match res {
        Some(_) => true,
        None => false
    };
    if res {
        println!("{} was successfully deleted", key);
    }
    else {
        println!("{} not found", key);
    }
}

fn list(db: &sled::Db){
    println!("Listing all entries");
    let mut iter = db.iter();
    let mut i = 0;
    loop {
        let row = match iter.next() {
            Some(r) => r,
            None => std::result::Result::Err(sled::Error::Unsupported(String::from(""))),
        };
        let (key, value) = match row {
            Ok((k, v)) => (k, v),
            _ => (IVec::from(""), IVec::from("")),
        };
        if key == IVec::from("") {
            break;
        }
        i+=1;
        println!("{}) {}: {}", i,
                 str::from_utf8(&key).unwrap(), str::from_utf8(&value).unwrap());
    }
    if i > 0 {
        println!("\nTotal entries: {}", i);
    }
    else {
        println!("Database is empty");
    }
}

fn parse_input(input: &String) -> Command{
    let s = input.trim();
    let s: Vec<&str>  = s.split(" ").collect();
    if s.len() > 3 || s.len() <= 0 {
        return Command::ERROR;
    }
    if s[0] == "HELP" {
        return Command::HELP;
    }
    if s[0] == "LIST" {
        return Command::LIST;
    }
    if s[0] == "EXIT" {
        return Command::EXIT;
    }
    if s[0] == "INSERT" {
        if s.len() < 3 {
            return Command::ERROR;
        }
        return Command::INSERT(s[1], s[2]);
    }
    if s[0] == "GET" {
        if s.len() < 2 {
            return Command::ERROR;
        }
        return Command::GET(s[1]);
    }
    if s[0] == "UPDATE" {
        if s.len() < 3 {
            return Command::ERROR;
        }
        return Command::UPDATE(s[1], s[2]);
    }
    if s[0] == "DELETE" {
        if s.len() < 2 {
            return Command::ERROR;
        }
        return Command::DELETE(s[1]);
    }
    Command::ERROR
}

fn show_help(){
    println!("You asked for help");
}