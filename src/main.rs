use core::panic;

use axum_assessment_web_server::App;

#[tokio::main]
async fn main() {
    let server = match App::new() {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Error creating server: {error}");
            panic!()
        }
    };
    match server.serve().await {
        Ok(_) => println!("Server exited successfully"),
        Err(error) => eprintln!("Server crashed with error: {error}"),
    }
}
