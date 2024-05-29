#[macro_use] extern crate rocket;
use std::path::{PathBuf, Path};
use rocket::fs::{NamedFile, TempFile};
use rocket::form::Form;
use std::fs;

#[derive(FromForm)]
struct FileUpload<'r> {
    file: TempFile<'r>,
}

fn get_extension_from_mime_type(mime_type: &str) -> &str {
    match mime_type {
        "text/plain" => "txt",
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "application/pdf" => "pdf",
        _ => "bin",
    }
}

#[post("/upload_file", data = "<form>")]
async fn upload_file(mut form: Form<FileUpload<'_>>) -> std::io::Result<&'static str> {
    match form.file.name() {
        Some(val) => {
            let content_type = form.file.content_type().unwrap().to_string();
            let file_name = format!("{}.{}", val, get_extension_from_mime_type(&content_type));
            let path: PathBuf = Path::new("bucket/").join(file_name);
            form.file.persist_to(&path).await?;
            Ok("File uploaded successfully.\n")
        },
        None => Ok("File has no name.\n"),
    }
}

#[get("/download_file/<file_name>")]
async fn download_file(file_name: PathBuf) -> Option<NamedFile> {
    let path = Path::new("bucket/").join(file_name);
    NamedFile::open(path).await.ok()
}

#[get("/get_file_names")]
fn get_file_names() -> std::io::Result<String> {
    let mut file_names: Vec<String> = Vec::new();
    for path in fs::read_dir("bucket/").unwrap() {
        match path.unwrap().path().to_str() {
            Some(file_name) => file_names.push(file_name.to_string()),
            None => continue,
        }
    }
    let result = format!("{:?}", file_names);
    Ok(result)
}

#[delete("/delete_files?<file_names>")]
fn delete_files(file_names: String) -> std::io::Result<&'static str> {
    for file in file_names.split(",") {
        let path = Path::new("bucket/").join(file);
        if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }
    Ok("Files have been deleted.\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/bucket", routes![upload_file])
        .mount("/bucket", routes![download_file])
        .mount("/bucket", routes![get_file_names])
        .mount("/bucket", routes![delete_files])
}