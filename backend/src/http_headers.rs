/// HEADERS (Hdr)
pub enum ContentTypeHdr {
    TextJavaScript,
    ApplicationJavaScript,
    TextHtml,
    TextCss,
    MultipareFormData,
    ApplicationJson,
}

impl ContentTypeHdr {
    fn get_content_type(&self) -> &str {
        match self {
            ContentTypeHdr::TextJavaScript => return "text/javascript",
            ContentTypeHdr::ApplicationJavaScript => return "application/javascript",
            ContentTypeHdr::TextHtml => return "text/html",
            ContentTypeHdr::TextCss => return "text/css",
            ContentTypeHdr::MultipareFormData => return "multipart/form-data",
            ContentTypeHdr::ApplicationJson => return "application/json",
        }
    }
}
