#[macro_use] extern crate rocket;
use std::path::{PathBuf, Path};
use rocket::fs::{NamedFile, TempFile};
use std::io::{Error, ErrorKind};
use rocket::form::Form;
use regex::Regex;
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
        "text/csv" => "csv",
        "application/msword" => "doc",
        "application/zip" => "zip",
        _ => "bin",
    }
}

fn sanatize_file_name(file_name: &str) -> String {
    let reg = Regex::new(r"[^a-zA-Z0-9_.-]").unwrap();
    reg.replace_all(file_name, "_").to_string()
}

#[post("/upload_file", data = "<form>")]
async fn upload_file(mut form: Form<FileUpload<'_>>) -> Result<&'static str, Error> {
    match form.file.name() {
        Some(name) => {
            match form.file.content_type() {
                Some(content_type) => {
                    let file_name: String = format!("{}.{}", sanatize_file_name(name), get_extension_from_mime_type(&content_type.to_string()));
                    let path: PathBuf = Path::new("bucket/").join(file_name);
                    form.file.persist_to(&path).await?;
                    Ok("File uploaded successfully.\n")
                }
                None => Err(Error::new(ErrorKind::InvalidInput, "File has no supported content type.\n"))
            }
        },
        None => Err(Error::new(ErrorKind::InvalidInput, "File has no name.\n"))
    }
}

#[get("/download_file/<file_name>")]
async fn download_file(file_name: PathBuf) -> Option<NamedFile> {
    let path = Path::new("bucket/").join(file_name);
    NamedFile::open(path).await.ok()
}

#[get("/get_file_names")]
fn get_file_names() -> Result<String, Error> {
    let mut file_names: Vec<String> = Vec::new();
    match fs::read_dir("bucket/") {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(path) => file_names.push(path.path().to_str().unwrap()[7..].to_string()),
                    Err(e) => return Err(Error::new(e.kind(), e))
                }
            }
        }
        Err(e) => return Err(Error::new(e.kind(), e))
    }
    Ok(format!("{:?}", file_names))
}

#[delete("/delete_files?<file_names>")]
fn delete_files(file_names: String) -> Result<&'static str, Error> {
    for file in file_names.split(",") {
        let path = Path::new("bucket/").join(file);
        if path.exists() {
            match fs::remove_file(path) {
                Ok(_) => continue,
                Err(e) => return Err(Error::new(e.kind(), e))
            }
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