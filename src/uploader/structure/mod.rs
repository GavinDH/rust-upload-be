use serde::Deserialize;

#[derive(Deserialize)]

pub struct UploadFileStruct {
    pub name:String,
    pub file:String,
}