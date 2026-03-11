use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Deserialize)]
struct InputText {
    text: String,
}

#[derive(Serialize)]
struct HashOutput {
    hash: String,
}

#[post("/process")]
async fn process_text(data: web::Json<InputText>) -> impl Responder {
    let mut hasher = Sha256::new();
    hasher.update(&data.text);
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);

    HttpResponse::Ok().json(HashOutput { hash: hash_string })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(process_text) // 🚨 THIS FIXES 404
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
