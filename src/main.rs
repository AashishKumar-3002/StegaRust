use rocket::{get, post, routes, launch};
use rocket::serde::{json::Json, Deserialize};
use rocket::response::status::Custom;
use rocket::http::Status;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

// Error and Result type aliases
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


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
            Custom(Status::BadRequest, Json(response))
        }
    }
}

// Status endpoint: /status
#[get("/status")]
fn status() -> Json<&'static str> {
    Json("API is running")
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
        status
    ])
}
