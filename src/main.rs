use actix_cors::Cors;
use actix_web::{get, web::Query, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::Deserialize;

fn get_id_token(req: HttpRequest) -> String {
    match req.headers().get("Authorization") {
        None => panic!("No Authorization header"),
        Some(auth_header) => {
            let auth_header_str = auth_header.to_str().unwrap();
            auth_header_str.replace("Bearer ", "")
        }
    }
}

#[derive(Debug, Deserialize)]
struct CredentialsParam {
    code: String,
}

#[get("/credentials")]
async fn get_credentials(param: Query<CredentialsParam>) -> impl Responder {
    let auth_client = saasus_sdk_rust::client::auth::create_client();
    let credentials = auth_client
        .credential_api()
        .get_auth_credentials(Some(&param.code), Some("tempCodeAuth"), None)
        .await
        .unwrap();
    HttpResponse::Ok().json(credentials)
}

#[get("/refresh")]
async fn refresh(req: HttpRequest) -> impl Responder {
    let refresh_token = match req.cookie("SaaSusRefreshToken") {
        None => return HttpResponse::Unauthorized().body("No refresh token found"),
        Some(cookie) => cookie.value().to_string(),
    };

    let auth_client = saasus_sdk_rust::client::auth::create_client();
    let credentials = auth_client
        .credential_api()
        .get_auth_credentials(None, Some("refreshTokenAuth"), Some(&refresh_token))
        .await
        .unwrap();
    HttpResponse::Ok().json(credentials)
}

#[get("/userinfo")]
async fn get_me(req: HttpRequest) -> impl Responder {
    let auth_client = saasus_sdk_rust::client::auth::create_client();
    let user_info = auth_client
        .user_info_api()
        .get_user_info(&get_id_token(req))
        .await
        .unwrap();
    HttpResponse::Ok().json(user_info)
}

#[get("/users")]
async fn get_users(req: HttpRequest) -> impl Responder {
    let auth_client = saasus_sdk_rust::client::auth::create_client();
    let user_info = auth_client
        .user_info_api()
        .get_user_info(&get_id_token(req))
        .await
        .unwrap();

    let tenant_id = &user_info.tenants[0].id;

    let users = auth_client
        .tenant_user_api()
        .get_tenant_users(tenant_id)
        .await
        .unwrap()
        .users;

    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .service(get_credentials)
            .service(refresh)
            .service(get_me)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
