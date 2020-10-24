use crate::uploader;

pub fn fly() {
    rocket::ignite()
        .mount("/upload", routes![uploader::upload])
        .launch();
}