// #![deny(unused_crate_dependencies)]
// #![deny(warnings)]

#![deny(unused_variables)]
use sha2::{Sha256, Digest};
use uuid::Uuid;
use actix_web::{web, App, HttpServer, HttpResponse, Error, HttpRequest, dev::ServiceRequest, dev::Service, dev::ServiceResponse, middleware, Responder};
use futures::future::{ok, Either, Ready};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};

// 定义结构体用于接收POST请求中的JSON数据
#[derive(Deserialize)]
struct InputData {
    data: String,
}
// JWT 结构
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // 用户ID
    exp: usize,   // 过期时间戳
}

// 处理POST请求的处理器函数
// #[post("/hash")]
async fn hash_data(input: web::Json<InputData>) -> impl Responder {
    // 创建SHA256摘要计算器
    let mut hasher = Sha256::new();
    
    // 将请求中的数据写入哈希计算器
    hasher.update(input.data.as_bytes());
    
    // 计算哈希值并转换为十六进制字符串
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    // 返回计算后的SHA256摘要
    HttpResponse::Ok().body(hash_hex)
}



// 用于验证的密钥
const SECRET: &[u8] = b"my_secret_key";

// 创建 JWT 的示例
fn create_jwt(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: 10000000000,  // 过期时间戳，通常为当前时间 + 有效期
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

// 受保护的资源
async fn protected_resource() -> HttpResponse {
    HttpResponse::Ok().body("This is a protected resource!")
}

// 开放的资源
async fn open_resource() -> HttpResponse {
    HttpResponse::Ok().body("This is an open resource!")
}

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
            // .wrap_fn(auth_middleware) // 添加中间件
            .route("/protected", web::get().to(protected_resource)) // 受保护的路由
            .route("/open", web::get().to(open_resource))           // 开放的路由
            .route("/", web::get().to(get_uuid))
            .route("/hash", web::post().to(hash_data))  
            // .service(hash_data)  // 注册处理POST请求的服务
    })
    .bind(("127.0.0.1", 8077))? 
    .run()
    .await
}
