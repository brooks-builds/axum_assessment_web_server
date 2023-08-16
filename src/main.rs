use axum_assessment_web_server::App;

#[tokio::main]
async fn main() {
    let server = App::new();
    match server.serve().await {
        Ok(_) => println!("Server exited successfully"),
        Err(error) => eprintln!("Server crashed with error: {error}"),
    }
}
