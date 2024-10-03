// --------------------------------------------
//          Web Programing Basics
// --------------------------------------------

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // v1
    // let stream = listener.accept();
    // println!(
    //     "The stream is: {:?} \n The socket is: {:?}",
    //     stream.as_ref().unwrap().1,
    //     stream.as_ref().unwrap().0
    // );

    // v2
    // for i in 0..10 {
    //     match listener.accept() {
    //         Ok((socket, addr)) => {
    //             println!("The client info is: {:?}", addr);
    //         }
    //         Err(e) => {
    //             println!("Couldn't get client: {:?}", e);
    //         }
    //     }
    // }

    // v3
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     // handle_connection(stream);
    //     // handle_connection_v2(stream);
    //     handle_connection_v3(stream);
    // }

    // v4
    let mut active_requests = Arc::new(Mutex::new(0));
    for stream in listener.incoming() {
        let active_requests = Arc::clone(&active_requests);
        let stream = stream.unwrap();

        thread::spawn(move || {
            {
                let mut connections = active_requests.lock().unwrap();
                *connections += 1;
                if *connections > 2 {
                    thread::sleep(Duration::from_secs(2));
                }
            }

            handle_connection_v3(stream);

            {
                let mut connections = active_requests.lock().unwrap();
                *connections -= 1;
            }
        });
    }
}

fn _handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    println!("The request is: {:#?}", http_request);

    /*
    Response Syntax

    HTTP-Version Status-Code Reason-Phrase CRLF
    headers CRLF
    message-body

    ex: HTTP/1.1 200 OK\r\n\r\n
    */
    let response = "HTTP/1.1 200 OK \r\n";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn _handle_connection_v2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    println!("The request is: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK \r\n";
    let contents = fs::read_to_string("src/bin/web/index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line} Content-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection_v3(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, file_name) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK \r\n", "src/bin/web/index.html"),
        "GET /page1 HTTP/1.1" => {
            /*
                Adding sleep to simulate a multiple request
                will be blocked by the page1 request.
            */
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK \r\n", "src/bin/web/page1.html")
        }
        "GET /page2 HTTP/1.1" => ("HTTP/1.1 200 OK \r\n", "src/bin/web/page2.html"),
        _ => ("HTTP/1.1 404 NOT FOUND \r\n", "src/bin/web/404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{status_line} Content-Length: {}\r\n\r\n{contents}",
        contents.len()
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
