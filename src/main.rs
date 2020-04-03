#[macro_use]
extern crate log;
extern crate reqwest;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use dotenv::dotenv;
use std::env;

mod post;

// #[get{"/"}]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello World!!!")
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(post::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    
    info!("Start server");
    server.run().await
}