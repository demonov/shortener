use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use crate::mocked_db::MockedDb;
use crate::db::Db;

mod db;
mod mocked_db;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

async fn add(req: HttpRequest, d: web::Data<MockedDb>) -> HttpResponse {
    let url = req.match_info().query("tail");
    match d.add(&url) {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e)),
    }
}

async fn get(web::Path(name): web::Path<String>, d: web::Data<MockedDb>) -> HttpResponse {
    match d.get(&name) {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(e) => HttpResponse::Ok().body(format!("Error: {:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MockedDb::new("shortener");
    HttpServer::new(move|| {
        App::new()
            .data(db.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/add/{tail:.*}").route(web::get().to(add)))
            .service(web::resource("/get/{key}").route(web::get().to(get)))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}