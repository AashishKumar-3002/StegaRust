use std::io::{ErrorKind, Read, Write};
use std::path::Path;
use rocket::data::ToByteUnit;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use std::fs::{self, File};
use rocket::{data, get, launch, options, post, routes, Request, Response};
use rocket::serde::{json::Json, Deserialize , Serialize};
use rocket::response::status::Custom;
use rocket::http::{ContentType, Header, Status};
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions};


mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

// Error and Result type aliases
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize)]
struct StatusResponse {
    encoding: &'static str,
    decoding: &'static str,
    metadata: &'static str,
}

#[derive(Deserialize, Debug)]
struct EncodeData {
    path: String,
    chunk_type: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct DecodeData {
    path: String,
    chunk_type: String,
}

#[derive(Deserialize, Debug)]
struct PrintData {
    path: String,
}

#[derive(Deserialize, Debug)]
struct RemoveData {
    path: String,
    chunk_type: String,
}

#[derive(rocket::serde::Serialize)]
struct PrintResponse {
    status: String,
    chunks: Option<Vec<String>>,
    message: String
}

#[derive(Serialize, Deserialize)]
struct UploadResponse {
    status: String,
    image_path: String,
    message: String,
}

// Root endpoint: /
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

// Encode endpoint: /encode
#[post("/encode", data = "<encode_data>")]
fn encode_endpoint(encode_data: Json<EncodeData>) -> Custom<Json<PrintResponse>> {
    let path = &encode_data.path;
    let chunk_type = &encode_data.chunk_type;
    let message = &encode_data.message;

    println!("Encoding Data:");
    println!("Path: {}", path);
    println!("Chunk Type: {}", chunk_type);
    println!("Message: {}", message);

    match commands::encode(path, chunk_type, message) {
        Ok(_) => {
            let response = PrintResponse {
                status: "success".to_string(),
                chunks: Some(vec![chunk_type.clone()]),
                message: "Encoding successful!".to_string(),
            };
            Custom(Status::Ok, Json(response))
        }
        Err(e) => {
            eprintln!("Error encoding: {}", e);
            let response = PrintResponse {
                status: "error".to_string(),
                chunks: None,
                message: e.to_string(),
            };
            Custom(Status::BadRequest, Json(response))
        }
    }
}

// Decode endpoint: /decode
#[post("/decode", data = "<decode_data>")]
fn decode_endpoint(decode_data: Json<DecodeData>) -> Custom<Json<PrintResponse>> {
    let path = &decode_data.path;
    let chunk_type = &decode_data.chunk_type;

    println!("Decoding Data:");
    println!("Path: {}", path);
    println!("Chunk Type: {}", chunk_type);

    match commands::decode(path, chunk_type) {
        Ok(message) => {
            let response = PrintResponse {
                status: "success".to_string(),
                chunks: Some(vec![chunk_type.clone()]),
                message,
            };
            Custom(Status::Ok, Json(response))
        }
        Err(e) => {
            eprintln!("Error decoding: {}", e);
            let response = PrintResponse {
                status: "error".to_string(),
                chunks: None,
                message: e.to_string(),
            };
            Custom(Status::BadRequest, Json(response))
        }
    }
}

// Print endpoint: /print
#[post("/print", data = "<print_data>")]
fn print_endpoint(print_data: Json<PrintData>) -> Custom<Json<PrintResponse>> {
    let img_path = &print_data.path;
    println!("Printing Path: {}", img_path);

    // Call the print function logic and handle the result
    match commands::print(img_path) {
        Ok(chunks) => {
            let response = PrintResponse {
                status: "success".to_string(),
                chunks: Some(chunks.clone()),
                message: format!("found {} chunks", chunks.len())
            };
            Custom(Status::Ok, Json(response)) // Return the chunks as JSON
        },
        Err(e) => {
            eprintln!("Error printing: {}", e);
            let response = PrintResponse {
                status: "error".to_string(),
                chunks: None,
                message: e.to_string()
            };
            Custom(Status::InternalServerError, Json(response))
        }
    }
}



// Remove endpoint: /remove
#[post("/remove", data = "<remove_data>")]
fn remove_endpoint(remove_data: Json<RemoveData>) -> Custom<Json<PrintResponse>> {
    let path = &remove_data.path;
    let chunk_type = &remove_data.chunk_type;

    println!("Removing Data:");
    println!("Path: {}", path);
    println!("Chunk Type: {}", chunk_type);

    match commands::remove(path, chunk_type) {
        Ok(_) => {
            let response = PrintResponse {
                status: "success".to_string(),
                chunks: Some(vec![chunk_type.clone()]),
                message: format!("Chunk {} removal successful!" , chunk_type),
            };
            Custom(Status::Ok, Json(response))
        }
        Err(e) => {
            eprintln!("Error removing chunk: {}", e);
            let response = PrintResponse {
                status: "error".to_string(),
                chunks: None,
                message: e.to_string(),
            };
            Custom(Status::Ok, Json(response))
        }
    }
}

#[post("/upload", data = "<data>")]
async fn upload(content_type: &ContentType , data: data::Data<'_>) -> Custom<Json<UploadResponse>> {

    if !content_type.is_form_data() {
        let response = UploadResponse {
            status: "error".to_string(),
            image_path: String::new(),
            message: format!("Not a valid format , ensure it's form data format"),
        };
        return Custom(Status::BadRequest, Json(response));
    }
    
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("image")
            .size_limit(u64::from(32.mebibytes()))
    ]);

    let multi_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();

    let file = multi_form_data.files.get("image");

    if let Some(file_fields) = file {
        let file_field = &file_fields[0];

        let filename = &file_field.file_name;
        let content_type = &file_field.content_type;

        println!("FileName: {:?}", filename);
        println!("Content_type: {:?}", content_type);

        // Define the directory path where images will be stored
        let images_dir = Path::new("./images");

        // Check if the directory exists, if not, create it
        if !images_dir.exists() {
            match fs::create_dir_all(images_dir) {
                Ok(_) => println!("Created images directory"),
                Err(e) => {
                    let response = UploadResponse {
                        status: "error".to_string(),
                        image_path: String::new(),
                        message: format!("Failed to create images directory: {}", e),
                    };
                    return Custom(Status::InternalServerError, Json(response));
                }
            }
        }

        // Construct the full file path where the image will be saved
        let file_path = images_dir.join(filename.as_ref().unwrap());

        // Open or create the destination file
        let mut file = match std::fs::File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                let response = UploadResponse {
                    status: "error".to_string(),
                    image_path: String::new(),
                    message: format!("Failed to create file: {}", e),
                };
                return Custom(Status::InternalServerError, Json(response));
            }
        };

        let path = &file_field.path;

        let mut temp_file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                let response = UploadResponse {
                    status: "error".to_string(),
                    image_path: String::new(),
                    message: format!("Failed to open temp file: {}", e),
                };
                return Custom(Status::InternalServerError, Json(response));
            }
        };

        let mut buffer = Vec::new();

        if let Err(e) = temp_file.read_to_end(&mut buffer) {
            let response = UploadResponse {
                status: "error".to_string(),
                image_path: String::new(),
                message: format!("Failed to read temp file: {}", e),
            };
            return Custom(Status::InternalServerError, Json(response));
        }

        if let Err(e) = file.write_all(&buffer) {
            let response = UploadResponse {
                status: "error".to_string(),
                image_path: String::new(),
                message: format!("Failed to write to file: {}", e),
            };
            return Custom(Status::InternalServerError, Json(response));
        }

        // Return success response with the image path
        let response = UploadResponse {
            status: "success".to_string(),
            image_path: file_path.to_string_lossy().to_string(),
            message: "File processed successfully.".to_string(),
        };
        return Custom(Status::Ok, Json(response));
    }

    let response = UploadResponse {
        status: "error".to_string(),
        image_path: String::new(),
        message: format!("Upload Failed"),
    };
    return Custom(Status::BadRequest, Json(response));
}



// Status endpoint: /status
#[get("/status")]
fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        encoding: "online",
        decoding: "online",
        metadata: "online",
    })
}

#[get("/download/<file_name>")]
async fn download_file(file_name: String) -> Option<NamedFile> {
    let file_path = format!("./images/{}", file_name);
    NamedFile::open(Path::new(&file_path)).await.ok()
}


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type, Accept",
        ));
    }
}

// Handle preflight requests
#[options("/<_..>")]
fn all_options() -> &'static str {
    ""
}

// Launch the Rocket server
#[launch]
pub fn rocket() -> _ {

    rocket::build().mount("/", routes![
        index, 
        encode_endpoint, 
        decode_endpoint, 
        print_endpoint, 
        remove_endpoint, 
        upload,
        download_file,
        status,
        all_options
    ]).attach(Cors)
}
