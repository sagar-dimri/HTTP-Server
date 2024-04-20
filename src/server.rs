use crate::concurrent::ThreadPool;
use crate::http::StatusCode;
use crate::http::Request;
use crate::http::Response;
use crate::http::ParseError;
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener, io::Write};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response { // default implementation still can be overridden
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server{
    pub fn new(addr : String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        // let thread_pool = ThreadPool::new(4);
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // let job =  || {
                            //         let response = match Request::try_from(&buffer[..]) {
                            //         Ok(request) => {
                            //             // dbg!(request);
                            //             // Response::new(
                            //             //     StatusCode::Ok, 
                            //             //     Some("<h1> IT WORKS </h1>".to_string()),
                            //             // )
                            //             handler.handle_request(&request)
                            //         },
                            //         Err(e) => {
                            //             // println!("Failed to parse a request: {}", e);
                            //             // Response::new(StatusCode::BadRequest, None)
                            //             handler.handle_bad_request(&e)
                            //         }
                            //     };
                            //     if let Err(e) = response.send(&mut stream) {
                            //         println!("Failed to send response: {}", e);
                            //     }
                            // };

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    // Response::new(
                                    //     StatusCode::Ok, 
                                    //     Some("<h1> IT WORKS </h1>".to_string()),
                                    // )
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    // println!("Failed to parse a request: {}", e);
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => print!("Failed to read from connection : {}", e),
                    } 
                        
                }
                Err(e) => print!("Failed to establish a connection : {}", e),

            }
        }
    }
}