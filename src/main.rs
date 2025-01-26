use actix_web::{web, App, HttpServer, Responder};
use serde_json::Value;
use std::collections::HashMap;

async fn hello(name: web::Path<String>) -> impl Responder {
    // 组装返回值
    let mut result = HashMap::new();
    result.insert("code", Value::Number(200.into()));
    result.insert("message", Value::String(format!("Hello, {}", name)));

    // 转换成 JSON 格式返回
    web::Json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务
    HttpServer::new(|| App::new().route("/hello/{name}", web::get().to(hello)))
        .bind("127.0.0.1:18086")?
        .run()
        .await
}
