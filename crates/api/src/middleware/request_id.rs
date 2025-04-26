use tower_http::request_id::{
    MakeRequestId, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _request: &http::Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

pub fn create_request_id_middleware() -> SetRequestIdLayer<UuidRequestId> {
    SetRequestIdLayer::new(
        "x-request-id",
        UuidRequestId::default(),
    )
}
