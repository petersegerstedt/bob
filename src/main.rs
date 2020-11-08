use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use log::{info};

#[get("/helo")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/push")]
async fn echo(req_body: String) -> impl Responder {
    info!("Request body: \n{}", &req_body);
    let msg: bob::gitea::Msg = serde_json::from_str(&req_body).unwrap();
    let res = serde_json::to_string_pretty(&msg).unwrap();
    info!("Parsed message: \n{}", res);
    HttpResponse::Ok()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(fs::Files::new("/", "./static").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
