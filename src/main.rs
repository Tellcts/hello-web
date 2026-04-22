use hello_web::{ThreadPool, handle_connection};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                println!("Error occured when handling connection: {} ", e);
                std::process::exit(1);
            }
        })
    }
}
