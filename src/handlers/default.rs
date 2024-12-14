use actix_web::{get, web, HttpResponse, Responder};
use std::path::PathBuf;

#[get("/{path:.*}")]
pub async fn unknown_file(path: web::Path<String>, web_root: web::Data<String>) -> impl Responder {
    let relative_path = path.into_inner();
    let mut absolute_path = PathBuf::from(web_root.get_ref());
    absolute_path.push(&relative_path);

    HttpResponse::Ok().body(format!(
        "Unknown file type for {}. Please check manually.",
        absolute_path.display()
    ))
}
