use crate::uploader::structure::UploadFileStruct;

pub fn upload_handler(upload_file: &UploadFileStruct) -> bool {
    //ToDo: Write real function
    println!("file :{} Name:{}", upload_file.file, upload_file.name);
    return true;
}