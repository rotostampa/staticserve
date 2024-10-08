use actix_files as fs;
use actix_web::http::header::ContentEncoding;
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use clap::Parser;
use std::io::Read;
use std::path::{Path, PathBuf};

const GZIP_MAGIC_BYTES: [u8; 2] = [0x1F, 0x8B];

/// Simple file server using Actix-Web
#[derive(Parser, Debug, Clone)] // Add Clone here
struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value_t = 9876)]
    port: u16,

    /// Directory where the files are served from
    #[arg(short, long, default_value = "./files")]
    folder: String,

    /// Host to bind the server to
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,
}

#[get("/{filename:.*}")]
async fn index(req: HttpRequest, data: actix_web::web::Data<Args>) -> Result<HttpResponse, Error> {
    let filename: PathBuf = if req.match_info().query("filename").is_empty() {
        PathBuf::new() // Root path "/"
    } else {
        Path::new(&data.folder).join(req.match_info().query("filename"))
    };

    // Check if the path is a directory or does not exist
    if filename.as_os_str().is_empty() || filename.is_dir() || !filename.exists() {
        // Return a custom 404 HTML response if it's the root, a directory, or the file does not exist
        return Ok(HttpResponse::NotFound().content_type("text/html").body(""));
    }

    // If the file exists, open it
    let mut file = fs::NamedFile::open(filename)?;

    // Read the first two bytes
    let mut buffer = [0; 2];
    if file.read_exact(&mut buffer).is_ok() && buffer == GZIP_MAGIC_BYTES {
        file = file.set_content_encoding(ContentEncoding::Gzip);
    }

    // Return the file as an HttpResponse
    Ok(file.use_last_modified(true).into_response(&req))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse the command line arguments
    let args = Args::parse();

    // Start the HTTP server with the given port and serve files from the provided directory
    let port = args.port;
    let host = args.host.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(args.clone())) // Share CLI args with the handler
            .service(index)
    })
    .bind((host, port))? // Now we can still use the port variable
    .run()
    .await
}
