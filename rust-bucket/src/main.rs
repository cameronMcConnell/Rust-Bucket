#[macro_use] extern crate rocket;

#[post("/upload_file")]
fn upload_file() -> () {

}

#[get("/download_file")]
fn download_file() -> () {

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