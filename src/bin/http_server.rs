use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};

use volo_gen::volo::example::{
	PingRequest,
	PingResponse,
	SetRequest,
	SetResponse,
	GetRequest,
	GetResponse,
	DelRequest,
	DelResponse,
    PubRequest,
    PubResponse,
    SubRequest,
    SubResponse,
};

use faststr::FastStr;
use serde::Deserialize;
use volo_gen::volo::example::{RedisServiceClient, RedisServiceClientBuilder};
use mini_redis::{FilterLayer};
use lazy_static::lazy_static;

type RpcClient = RedisServiceClient;
type RpcClientBuilder = RedisServiceClientBuilder;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // rpc 通信的端口是 8080
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let rpc_cli = RpcClientBuilder::new("miniredis").address(addr).build();

    let app = Router::new()
                .route("/ping", get(ping))
                .route("/get/:keys", get(get_key).with_state(rpc_cli.clone()))
                .route(
                    "/set",
                    get(show_set_form).post(set_key).with_state(rpc_cli.clone()),
                )
                .route("/del", get(show_del_form).post(del_key).with_state(rpc_cli.clone()));
    
    // Web Server 的端口是 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await.unwrap();
}

async fn ping() -> (StatusCode, &'static str) {
    (StatusCode::OK, "PONG")
}

async fn get_key(Path(key): Path<String>, State(rpc_cli): State<RpcClient>) -> Response {
    let resp = rpc_cli.get(GetRequest { id: 0, key: key.into() }).await.unwrap();
    if resp.value == None {
        (StatusCode::NOT_FOUND, "not found").into_response()
    } else {
        let val = String::from(resp.value.unwrap());
        (StatusCode::OK, val).into_response()
    }
}

#[derive(Deserialize, Debug)]
struct FormSetKey {
    key: String,
    value: String,
}

async fn show_set_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <label for="value">
                        Enter value:
                        <input type="text" name="value">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn set_key(State(rpc_cli): State<RpcClient>, Form(setkey): Form<FormSetKey> ) -> Response {
    rpc_cli.set(SetRequest { id: 0, key: setkey.key.into(), value: setkey.value.into() }).await.unwrap();
    (StatusCode::OK, "set ok").into_response()
}

async fn show_del_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
struct FormDelKey {
    key: String,
}

async fn del_key(
    State(rpc_cli): State<RpcClient>,
    Form(delkey): Form<FormDelKey>,
) -> Response {
    let mut keys = Vec::new();
    keys.push(FastStr::new(delkey.key.as_str()));
    let resp = rpc_cli.del(DelRequest { id: 0, keys: keys }).await.unwrap();
    
    (StatusCode::OK, "ok").into_response()
}