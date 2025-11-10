use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
// async fn greet(req:HttpRequest) -> impl Responder{
//     let name = req.match_info().get("name").unwrap_or("world");
//     format!("Hello {}!", &name)
// }
async fn health_check(_req:HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server,std::io::Error> {
    let server = HttpServer::new(||{
        App::new()
        .route("/health_check", web::get().to(health_check))
        
    })
    .bind("127.0.0.1:8000")?
    .run();
    Ok(server)
}