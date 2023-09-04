use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Bind to address failed.");
    _ = run(listener);
}
