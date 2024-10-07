use actix_files as fs;
use actix_web::http::header::{ContentDisposition, ContentEncoding, DispositionType};
use actix_web::{get, App, Error, HttpRequest, HttpServer, Result};
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
}

#[get("/{filename:.*}")]
async fn index(req: HttpRequest, data: actix_web::web::Data<Args>) -> Result<fs::NamedFile, Error> {
    let filename: PathBuf = Path::new(&data.folder).join(req.match_info().query("filename"));

    let mut file = fs::NamedFile::open(filename)?;

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
    // Parse the command line arguments
    let args = Args::parse();

    // Start the HTTP server with the given port and serve files from the provided directory
    let port = args.port; // Store the port separately
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(args.clone())) // Share CLI args with the handler
            .service(index)
    })
    .bind(("127.0.0.1", port))? // Now we can still use the port variable
    .run()
    .await
}
