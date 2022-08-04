use rocket::serde::{Deserialize, Serialize};
use crate::schema::admin;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{ Serialize, Deserialize };

use bcrypt::{ DEFAULT_COST, hash, verify };

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "admin"]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub accessToken: String,
}

#[derive(Insertable)]
#[table_name = "admin"]
pub struct InsertableAdmin {
    pub username: String,
    pub password: String,
    pub accessToken: String
}

impl InsertableAdmin {
    fn from_admin(admin: Admin) -> InsertableAdmin {
        InsertableAdmin {
            username: admin.username,
            password: admin.password,
            name: admin.accessToken,
        }
    }
}

impl Admin {
    pub fn create(admin: Admin, connection: &PgConnection) -> QueryResult<Admin> {
        let encrypted_admin = Admin {
            password: hash(user.password,DEFAULT_COST).unwrap(),
            ..user
        };
        diesel::insert_into(admins::table)
        .values(&InsertableAdmin::from_admin(encrypted_admin))
        .execute(connection)?;

        admins::table.order(admins::id.desc()).first(connection)
    }

    pub fn get_by_username_and_password(username_: String, password_: String, connection: &PgConnection) -> Option<Admin> {
        let res = admins::table
            .filter(admins::username.eq(username_))
            .get_result::<Admin>(connection);
        match res {
            Ok(admin) => {
                if let Ok(matching) = verify(&password_,&admin.password) {
                    if matching {
                        return Some(admin)
                    }
                }
                return None
            }
            Err(_) => {
                None
            }
        }
    }
}