use serde::Serialize;
use crate::response::Response;
use crate::request::Request;

#[derive(Serialize)]
pub struct Model {
    id: i32
}

pub fn index(_request: Request) -> Response {
    let model = Model { id: 10 };

    if model.id > 8 {
        return Response::error();
    }


    Response::json(model)
}