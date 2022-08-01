use crate::schema::host;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{ Serialize, Deserialize };

use bcrypt::{ DEFAULT_COST, hash, verify };
#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "host"]
pub struct Host {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub password: String,
    pub accessToken: String,
    pub lastMeetingAt: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "host"]
pub struct InsertableHost {
    pub name: String,
    pub username: String,
    pub password: String,
    pub accessToken: String,
    pub lastMeetingAt: chrono::NaiveDateTime,
}

impl InsertableHost {
    fn from_host(host: Host) -> InsertableHost {
        InsertableHost {
            name: host.name,
            username: host.username,
            password: host.password,
            accessToken: host.accessToken,
            lastMeetingAt: host.lastMeetingAt,
        }
    }
}

impl Host{
    pub fn create(host: Host, connection: &PgConnection) -> QueryResult<Host> {
        let encrypted_host = Host {
            password: hash(host.password,DEFAULT_COST).unwrap(),
            ..host
        };
        diesel::insert_into(host::table)
        .values(&InsertableHost::from_host(encrypted_host))
        .execute(connection)?;

        host::table.order(host::id.desc()).first(connection)
    }
    
    pub fn get_by_username_and_password(username_: String, password_: String, connection: &PgConnection) -> Option<Host> {
        let res = host::table
            .filter(host::username.eq(username_))
            .get_result::<Host>(connection);
        match res {
            Ok(host) => {
                if let Ok(matching) = verify(&password_,&host.password) {
                    if matching {
                        return Some(host)
                    }
                }
            }
            Err(_) => {
                return None
            }
        }
    }
}