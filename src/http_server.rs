
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::collections::HashSet;
use std::fs;


struct HttpRequest {
    method : String,
    endpoint : String,
}

impl HttpRequest {
    pub fn new(request : &str) -> HttpRequest {
        HttpRequest {
            method: Self::parse_method(request.lines().nth(0).unwrap()),
            endpoint: Self::endpoint_method(request.lines().nth(0).unwrap())
        }
    }

    fn parse_method(request : &str) -> String {
        let method_end = request.find(" ").unwrap();
        let method : String = request.get(..method_end)
            .unwrap()
            .to_string();

        method
    }

    fn endpoint_method(request : &str) -> String {
        let method_end = request.find(" ").unwrap();
        let endpoint_end = request.get(method_end + 1..)
            .unwrap()
            .find(" ")
            .unwrap();

        let endpoint = request.get(method_end+1..).unwrap()
            .get(..endpoint_end)
            .unwrap()
            .to_string();

        endpoint
    }
}

pub struct ServerContext {
    listener: TcpListener,
    host_name : String,
    server_doc_set : HashSet<String>
}

impl ServerContext {
    pub fn new(host_name: &str) -> ServerContext {
        ServerContext {
            listener : TcpListener::bind(host_name).unwrap(),
            host_name : host_name.to_string(),
            server_doc_set : Self::pars_doc("./server"),
        }
    }
        
    pub fn run(&self) {
        println!("[INFO] Server run");
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let request = self.get_request(&mut stream);
            println!("{}", request);
            let http_req = HttpRequest::new(&request);

            Self::handle_request(&mut stream, http_req);
        }


    }
    
    fn pars_doc(path: &str) -> HashSet<String> {
        let mut files_set = HashSet::<String>::new();
        for file in fs::read_dir(path).unwrap() {
            let file_name  = file
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();

            files_set.insert(file_name.clone());
        }

        files_set
    } 

    fn get_request(&self, mut stream: &TcpStream) -> String {
        let mut buff = [0; 512];
        let size = stream.read(&mut buff);
        let recv_msg = String::from_utf8_lossy(&buff[..size.unwrap()]);

        recv_msg.to_string()
    }
    
    fn handle_request(mut stream: &TcpStream, request: HttpRequest) { 
        let path = format!("./server/{}",request.endpoint);

        let mut status_line = "HTTP/1.1 200 OK";
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => {
                status_line = "HTTP/1.1 404 Not Found";
                fs::read_to_string("./server/404.html").unwrap()
            }
        };

        let length = content.len();
        let respone = 
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");    

        stream.write_all(respone.as_bytes()).unwrap();
    }
}



