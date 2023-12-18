use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_index() -> impl Responder {
  HttpResponse::Ok().body("Manually saying hi!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(index)
      .service(echo)
      .route("/api", web::get().to(manual_index))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}