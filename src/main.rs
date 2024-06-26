mod device;
mod server;

use server::server::Server;
fn main() {
    let server = Server::new("127.0.0.1", 7878);

    let res = server.start();
    match res {
        Ok(_) => println!("Server started"),
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}
