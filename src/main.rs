use actix_web::{ get, post, App, HttpResponse, HttpServer, Responder };
use futures::try_join;

#[get("/")]
async fn api_index() -> impl Responder {
  HttpResponse::Ok().body("Hello world from API")
}

#[post("/echo")]
async fn api_echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn portal_index() -> impl Responder {
  HttpResponse::Ok().body("Portal goes here!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Create the API server
  let api_server = HttpServer::new(|| {
    App::new()
      .service(api_index)
      .service(api_echo)
  })
  .bind(("0.0.0.0", 9000))?;

  // Create the portal server
  let portal_server = HttpServer::new(|| {
    App::new()
      .service(portal_index)
  })
  .bind(("0.0.0.0", 9090))?;

  // Start both the servers concurrently
  try_join!(api_server.run(), portal_server.run())?;

  Ok(())
}