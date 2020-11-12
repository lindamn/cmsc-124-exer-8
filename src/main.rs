//Penaranda, Lindsay Anne D.
//T-5L

extern crate regex;
use regex::Regex;
use std::error::Error; 
use std::fs::File; 
use std::io::prelude::*; 
use std::path::Path; 
use std::io;

fn main(){
    let path = Path::new("files/input.arnoldc");
    let display = path.display();
    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn ’t open {}: {}", display, why.description()),
        Ok(file) => file,
    };//opens file read

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => print!(""),
    };//transfers file content to a string

    //regexes
    let mut key_vec = Vec::new();
    let mut num_vec = Vec::new();
    let mut str_vec = Vec::new();
    let mut nonkey_vec = Vec::new();
    let key_re = Regex::new(r#"^[A-Z]+$"#).unwrap();//to match keywords
    let num_re = Regex::new(r"\b-?\d+\b").unwrap();//to match numbers/integers
    let str_re = Regex::new(r#"^".*"$"#).unwrap();//to match strings
    let nonkey_re = Regex::new(r#"\b[a-z0-9]+\b"#).unwrap();//to match identifiers
    
    let re = Regex::new(r"IT'S SHOWTIME|YOU HAVE BEEN TERMINATED|TALK TO THE HAND .+|HEY CHRISTMAS TREE .+|YOU SET US UP .+|GET TO THE CHOPPER .+|HERE IS MY INVITATION.+|GET UP .+|GET DOWN .+|YOU'RE FIRED .+|HE HAD TO SPLIT .+|YOU ARE NOT YOU YOU ARE ME .+|LET OFF SOME STEAM BENNET .+|CONSIDER THAT A DIVORCE .+|KNOCK KNOCK .+|ENOUGH TALK").unwrap();
    for cap in re.captures_iter(&mut s){
        let mut token = cap.at(0).unwrap_or("");
        // println!("token: {}", token);
        let re2 = Regex::new(r#"(?P<k>[A-Z'\s]+) (?P<v>.+$)"#).unwrap();//splits the string into keyword and value
        for cap2 in re2.captures_iter(&mut token){
            let mut key = cap2.name("k").unwrap_or("").to_owned();
            let mut value = cap2.name("v").unwrap_or("").to_owned();
            if key_re.is_match(&value){ //if value is actually part of the keyword
                key.push_str(" ");
                key.push_str(&value);
                key_vec.push(key);
            }else if num_re.is_match(&value){ //if value is number/integer
                num_vec.push(value);
                key_vec.push(key);
            }else if str_re.is_match(&value){ // if value is string
                value.retain(|c| c != '"');//removes the double quotes
                str_vec.push(value);
                key_vec.push(key);
            }else if nonkey_re.is_match(&value){ //if value is identifier na hindi keyword
                nonkey_vec.push(value);
                key_vec.push(key);
            };
        };
    };

    let mut choice = String::new();
    println!("[1] Show all numbers");
    println!("[2] Show all keywords");
    println!("[3] Show all strings");
    println!("[4] Show all non-keyword identifiers");
    println!("Enter input:");
    io::stdin().read_line(&mut choice);
    let choice:i8 = choice.trim().parse().expect("error");

    if choice == 1{
        let mut len = num_vec.len();
        println!("Count: {}", len);
        for i in 0..len{
            println!("Detected integer: {}", num_vec[i]);
        }
    }else if choice == 2{
        let mut len = key_vec.len();
        println!("Count: {}", len);
        for i in 0..len{
            println!("Detected keyword: {}", key_vec[i]);
        }
    }else if choice == 3{
        let mut len = str_vec.len();
        println!("Count: {}", len);
        for i in 0..len{
            println!("Detected string literal: {}", str_vec[i]);
        }
    }else if choice == 4{
        let mut len = nonkey_vec.len();
        println!("Count: {}", len);
        for i in 0..len{
            println!("Detected keyword: {}", nonkey_vec[i]);
        }
    }else{
        println!("Invalid input!");
    }
}