extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::{Body, Response, StatusCode};

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

#[derive(Serialize)]
struct MoveResponse {
    #[serde(rename = "move")]
    direction: &'static str,
}

#[derive(Serialize)]
struct StartResponse {
    color: &'static str,
}

impl IntoResponse for MoveResponse {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized move response"),
        )
    }
}

impl IntoResponse for StartResponse {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized move response"),
        )
    }
}

fn move_handler(state: State) -> (State, MoveResponse) {
    let move_resp = MoveResponse {
       direction: "right",
    };

    (state, move_resp)
}

fn start_handler(state: State) -> (State, StartResponse) {
    let start_resp = StartResponse {
        color: "#ff9944",
    };

    (state, start_resp)
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/move").to(move_handler);
        route.post("/move").to(move_handler);
        route.get("/start").to(start_handler);
        route.post("/start").to(start_handler);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
