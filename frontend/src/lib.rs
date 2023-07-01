use actix_files::{Files};

pub fn create_service(path: &'static str) -> Files {
  Files::new(path, "./frontend/dist/")
        .prefer_utf8(true)
        .index_file("index.html")
}