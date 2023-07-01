use actix_files::{Files};

pub fn create_service<'a>(url_path: &'static str, dir_path: Option<String>) -> Files {
  let dir = dir_path.unwrap_or("./frontend/dist/".to_string());
  Files::new(url_path, dir)
        .prefer_utf8(true)
        .index_file("index.html")
}