/// POST
/// GET
/// PUT
/// DELETE
pub enum HttpRequest {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpRequest {
    fn get_method(&self) -> &str {
        match self {
            HttpRequest::GET => return "GET",
            HttpRequest::POST => return "POST",
            HttpRequest::PUT => return "PUT",
            HttpRequest::DELETE => return "DELETE",
        };
    }
}
