use crate::schema::privacy;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{ Serialize, Deserialize };

use bcrypt::{ DEFAULT_COST, hash, verify };

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "privacy"]
pub struct Privacy {
    pub id: i32,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "privacy"]
pub struct InsertablePrivacy {
    pub body: String,
}

impl InsertablePrivacy {
    fn from_privacy(privacy: Privacy) -> InsertablePrivacy {
        InsertablePrivacy {
            body: privacy.body,
        }
    }
}

impl Privacy {
    pub fn create(privacy: Privacy, connection: &PgConnection) -> QueryResult<Privacy> {
        diesel::insert_into(privacy::table)
        .values(&InsertablePrivacy::from_privacy(privacy))
        .execute(connection)?;

        privacy::table.order(privacy::id.desc()).first(connection)
    }

    pub fn get_by_id(id_: i32, connection: &PgConnection) -> Option<Privacy> {
        privacy::table
            .filter(privacy::id.eq(id_))
            .get_result(connection)
            .optional()
            .unwrap()
    }
}