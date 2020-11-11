extern crate regex;
use regex::Regex;
use std::error::Error; // Error class
use std::fs::File; // File class
use std::io::prelude::*; // for file reading (io)
use std::path::Path; // Path class
use std::io;

fn main(){
    let path = Path::new("files/input.arnoldc");
    let display = path.display();
    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn ’t open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    };
    //di ko pa nalalagay yung if statements and fxns hehey
    let re = Regex::new(r"^IT'S SHOWTIME|YOU HAVE BEEN TERMINATED|TALK TO THE HAND ([.]+)|HEY CHRISTMAS TREE ([.]+)|YOU SET US UP ([.]+)|GET TO THE CHOPPER ([.]+)|HERE IS MY INVITATION([.]+)|GET UP ([.]+)|GET DOWN ([.]+)|YOU'RE FIRED ([.]+)|HE HAD TO SPLIT ([.]+)|YOU ARE NOT YOU YOU ARE ME ([.]+)|LET OFF SOME STEAM BENNET ([.]+)|CONSIDER THAT A DIVORCE ([.]+)|KNOCK KNOCK ([.]+)|ENOUGH TALK$");
    for cap in re.captures_iter(s){
        println!("idk kung ano to: {}", cap.at(1).unwrap_or(""));
    }
}