use actix_files as fs;
use actix_files::NamedFile;
use actix_web::http::header::{
    ContentDisposition, ContentEncoding, DispositionType, HeaderName, HeaderValue,
};
use actix_web::{
    error::ErrorNotFound, get, http::StatusCode, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const GZIP_MAGIC_BYTES: [u8; 2] = [0x1F, 0x8B];

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf =
        PathBuf::from(format!("./files/{}", req.match_info().query("filename")));
    let mut file = fs::NamedFile::open(path)?;

    let mut buffer = [0; 2]; // Read the first two bytes
    if file.read_exact(&mut buffer).is_ok() && buffer == GZIP_MAGIC_BYTES {
        file = file.set_content_encoding(ContentEncoding::Gzip);
    }

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 9876))?
        .run()
        .await
}
