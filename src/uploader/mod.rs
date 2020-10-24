mod service;
mod structure;

use rocket::http::Status;
use rocket_contrib::json::Json;

use structure::UploadFileStruct;

#[post("/", format = "application/json", data = "<upload_file>")]
pub fn upload(upload_file: Json<UploadFileStruct> ) -> Status{

    if service::upload_handler(&upload_file.into_inner()){
        return Status::Ok
    }
    return Status::BadRequest

}