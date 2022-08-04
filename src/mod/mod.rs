pub mod model;
pub mod schema;
pub mod auth;

use self::model::Admin;
use database;
use self::auth::ApiKey;
use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{
    Header,
    Registered,
    Token,
};

#[post("/", data = "<admin>")]
fn create(admin: Json<Admin>, connection: db::Connection) -> Result<Json<Admin>, Status> {
    let insert = Admin { id: None, ..user.into_inner() };
    Admin::create(insert, &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/", data = "<admin>")]
fn read(admin: Json<Admin>, connection: db::Connection) -> Result<Json<Admin>, Status> {
    Admin::read(admin.into_inner().id, &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/admin/signin", data = "<admin>")]
fn signin(admin: Json<Admin>, connection: db::Connection) -> Result<Json<Admin>, Status> {
    let admin = admin.into_inner();
    let password = admin.password.clone();
    let password_hash = Sha256::hash(&password);
    let admin = Admin::read_by_username(admin.username, &connection)?;
    if admin.password_hash == password_hash {
        Ok(Json(admin))
    } else {
        Err(Status::Unauthorized)
    }
}

#[get("/admin/auth", data = "<admin>")]
fn auth(admin: Json<Admin>, connection: db::Connection) -> Result<Json<Admin>, Status> {
    let admin = admin.into_inner();
    let admin = Admin::read_by_username(admin.username, &connection)?;
    Ok(Json(admin))
}

