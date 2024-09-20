/*
 * This is entry point for rust server.
 *
 */
mod http_server;

fn main() {
    let server_endpoint = "127.0.0.1:6969";
    println!("[INFO] run server on {}", server_endpoint);
    let server_instance = http_server::ServerContext::new("127.0.0.1:6969");
    server_instance.run();
}



