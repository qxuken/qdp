use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};

pub fn create_service<'a>(url_path: &'static str, dir_path: Option<String>) -> Files {
    let dir = dir_path.unwrap_or("./frontend/dist/".to_string());
    Files::new(url_path, &dir.to_owned())
        .prefer_utf8(true)
        .use_last_modified(true)
        .index_file("index.html")
        .default_handler(fn_service(move |req: ServiceRequest| {
            let not_found_page_path = dir.clone() + "404.html";
            async move {
                let (req, _) = req.into_parts();
                let file = NamedFile::open_async(not_found_page_path).await?;
                let res = file.into_response(&req);
                Ok(ServiceResponse::new(req, res))
            }
        }))
}
