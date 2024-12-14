use actix_web::{get, web, HttpResponse, Responder};
use futures_util::stream::{self, StreamExt};
use mime_guess;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;

#[get("/{path:.*}")]
pub async fn serve_file(path: web::Path<String>, web_root: web::Data<String>) -> impl Responder {
    let relative_path = path.into_inner();
    let mut absolute_path = PathBuf::from(web_root.get_ref());
    absolute_path.push(&relative_path);

    // Check if the file exists
    if !absolute_path.exists() {
        return HttpResponse::NotFound()
            .body(format!("File {} does not exist", absolute_path.display()));
    }

    if absolute_path.is_dir() {
        return HttpResponse::Forbidden().body(format!(
            "{} is a directory, not a file",
            absolute_path.display()
        ));
    }

    // Determine file type
    let mime = mime_guess::from_path(&absolute_path).first_or_octet_stream();

    // Open the file
    match File::open(&absolute_path) {
        Ok(file) => {
            // Wrap the file in a buffered reader
            let reader = BufReader::new(file);

            // Create a stream from the file's data
            let stream = stream::unfold(reader, |mut reader| async {
                let mut buffer = [0; 8192];
                match reader.read(&mut buffer) {
                    Ok(0) => None, // EOF
                    Ok(n) => Some((Ok(web::Bytes::copy_from_slice(&buffer[..n])), reader)),
                    Err(err) => Some((
                        Err::<web::Bytes, std::io::Error>(io::Error::new(
                            io::ErrorKind::Other,
                            err,
                        )), // Explicitly using io::Error
                        reader,
                    )),
                }
            });

            HttpResponse::Ok()
                .content_type(mime.to_string())
                .streaming(stream)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error reading file"),
    }
}
