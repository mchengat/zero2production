use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscription(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
