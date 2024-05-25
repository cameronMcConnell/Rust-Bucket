#[macro_use] extern crate rocket;
use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;

#[post("/upload_file")]
fn upload_file() -> () {

}

#[get("/download_file/<file_name>")]
async fn download_file(file_name: PathBuf) -> Option<NamedFile> {
    let path = Path::new("bucket/").join(file_name);
    NamedFile::open(path).await.ok()
}

#[get("/get_file_names")]
fn get_file_names() -> () {

}

#[delete("/delete_files")]
fn delete_files() -> () {

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/bucket", routes![upload_file])
        .mount("/bucket", routes![download_file])
        .mount("/bucket", routes![get_file_names])
        .mount("/bucket", routes![delete_files])
}