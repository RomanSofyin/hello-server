use hello_server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // The commented out lines below dont work as `starts_with` in let-if-elseif seem to not work for some reason :-/ 
    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // let (status_line, filename) = if request_line.starts_with(get) {
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./src/hello.html")
    // } else if request_line.starts_with(sleep) {
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
