use askama::{Error, Template};
use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use html_minifier::HTMLMinifier;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        let mut minifier = HTMLMinifier::new();
        match self
            .0
            .render()
            .and_then(|html| minifier.digest(html).map_err(|e| Error::Custom(e.into())))
            .map(|_| minifier.get_html().to_vec())
            .and_then(|html_vec| String::from_utf8(html_vec).map_err(|e| Error::Custom(e.into())))
        {
            Ok(html) => (
                [
                    (
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("text/html; charset=utf-8"),
                    ),
                    (
                        header::CACHE_CONTROL,
                        HeaderValue::from_static("max-age:300, private"),
                    ),
                ],
                html,
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
