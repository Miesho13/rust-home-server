use walkdir::WalkDir;
use std::fs;

// use std:: {
//     io::{prelude::*, BufReader},
//     net::{TcpListener, TcpStream},
//     path::Path
// };

fn main() {

    let mut path = Vec::new();
    for entry in WalkDir::new("./server") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            path.push(entry.path().to_path_buf());
        }
    }

    for path in &path {
        match path.to_str() {
            Some(s) => println!("{}", s),
            None => println!("none unicode path"),
        }
    } 
}


