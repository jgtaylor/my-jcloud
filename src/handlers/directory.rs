use actix_web::{get, web, HttpResponse, Responder};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// List the contents of a directory
#[get("/{path:.*}")]
pub async fn list_directory(
    path: web::Path<String>,
    web_root: web::Data<String>,
) -> impl Responder {
    let relative_path = path.into_inner();
    let mut absolute_path = PathBuf::from(web_root.get_ref());
    absolute_path.push(&relative_path);

    // Check if the path exists and is a directory
    if !absolute_path.exists() {
        return HttpResponse::NotFound()
            .body(format!("Path {} does not exist", absolute_path.display()));
    }

    if !absolute_path.is_dir() {
        return HttpResponse::Forbidden().body(format!(
            "Path {} is not a directory",
            absolute_path.display()
        ));
    }

    // Generate the directory listing
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><title>Directory Listing</title></head><body>");
    html.push_str("<h1>Directory Listing</h1><ul>");

    for entry in WalkDir::new(&absolute_path)
        .max_depth(1)
        .into_iter()
        .flatten()
    {
        let path = entry.path();
        let display_name = path.file_name().unwrap_or_default().to_string_lossy();
        let is_dir = path.is_dir();

        let icon = if is_dir {
            "📁" // Icon for directories
        } else {
            "📄" // Icon for files
        };

        html.push_str(&format!(
            "<li>{icon} <a href=\"/{}/\">{}</a></li>",
            path.strip_prefix(&absolute_path).unwrap_or(path).display(),
            display_name
        ));
    }

    html.push_str("</ul></body></html>");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
