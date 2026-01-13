use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread};

mod thread_pool;
mod http_headers;
mod http_methods;

use crate::thread_pool::ThreadPool;
use crate::http_headers::ContentTypeHdr;
use crate::http_methods::HttpRequest;

fn main() {
    //TODO: read url from config file
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming() {
        //TODO: add error handling
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename, content_type) = match &request_line[..] {
        // page / endpoint
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "../frontend/html/index.html", ""),
        "GET /dist/index.js HTTP/1.1" => ("HTTP/1.1 200 OK", "../frontend/dist/index.js", "application/javascript"),
        "GET /dist/index.js.map HTTP/1.1" => ("HTTP/1.1 200 OK", "../frontend/dist/index.js.map", "application/javascript"),
        "GET /favicon.ico HTTP/1.1" => ("HTTP/1.1 204 No Content", "", ""),
        // filename.contains("dist") => (),
        // dist static files
        path if path.contains("/dist") => {
            let static_file: &str = path.split(" ").nth(1).map(|p| format!("../frontend/{}", p)).unwrap_or_default();
            ("HTTP/1.1 200 OK", static_file, "")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "", "text/html") //"../frontend/html/404.html"
    };
    
    // let (status_line, filename, content_type) = match &request_line[..] {
    //     line if line.starts_with("GET ") && line.contains("HTTP/1.1") => {
            
    //     }
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html"),
    // };

    let contents: String = get_contents(filename);
    let length: usize = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}

fn get_contents(filename: &str) -> String {
    if filename == "" {
        return String::from("")
    } else {
        return fs::read_to_string(filename).unwrap();
    }
}
