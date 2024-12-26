
use axum::Router;

use tower_http::services::ServeDir;

use photon_rs::channels::alter_blue_channel;
use photon_rs::native::{open_image,save_image};

#[tokio::main]
async fn main() {
    // Image processing Test code
    let mut img = open_image("./Test_images/in.jpg").expect("File should open");
    alter_blue_channel(&mut img, 25_i16);
    save_image(img, "./Test_images/out.jpg").expect("Unable to save");
    
    let app = Router::new().route_service("/images", ServeDir::new("images"));

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



