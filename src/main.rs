mod file_system;
use file_system::*;

use std:: {
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path
};

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:1314") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("[ERR] Error TcpListener:: {}", e);
            return; 
        }
    };

    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    
    let  (status_line, filename) = if request_line  == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    }
    else if request_line == "GET /index.css HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.css")
    }
    else if request_line == "GET /todo HTTP/1.1" {
        ("HTTP/1.1 200 OK", "todo/todo.html")
    }
    else if request_line == "GET /todo.css HTTP/1.1" {
        ("HTTP/1.1 200 OK", "todo/todo.css")
    }
    else if request_line == "GET /404.css HTTP/1.1" {
        ("HTTP/1.1 200 OK", "404.css")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let path = format!("./server/{filename}");
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let respone = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(respone.as_bytes()).unwrap();
}

