mod device;
mod server;

use server::server::Server;
fn main() {
    let server = Server::new("0.0.0.0", 7878); // Allow All IP

    let res = server.start();
    match res {
        Ok(_) => println!("Server started"),
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}
