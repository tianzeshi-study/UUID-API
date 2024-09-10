use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};
use uuid::Uuid;

// 处理 GET 请求并返回 UUID
// #[get("/get-uuid")]
// async fn get_uuid() -> impl Responder {
async fn get_uuid() -> HttpResponse {
    let uuid = Uuid::new_v4(); // 生成一个UUID v4
    let uuid_string = format!("{}", uuid);
    HttpResponse::Ok().body(uuid_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动 HTTP 服务器
    HttpServer::new(|| {
        App::new()
            // .service(get_uuid) // 注册 /get-uuid 路由
            .route("/", web::get().to(get_uuid))
    })
    // .bind(("0.0.0.0", 8077))? // 绑定到本地端口 8080
    .bind(("127.0.0.1", 8077))? 
    .run()
    .await
}
