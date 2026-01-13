pub trait Request {
    fn handle_request(self) -> Self {}
}