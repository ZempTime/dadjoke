use serde::{Serialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::{
  dadjokes,
  dadjokes::dsl::{dadjokes as all_dadjokes},
};

#[derive(Debug, Insertable)]
#[table_name = "dadjokes"]
pub struct NewDadjoke {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct Dadjoke {
  pub id: String,
  pub text: String,
}

impl Dadjoke {
  pub fn all(conn: &PgConnection) -> QueryResult<Vec<Dadjoke>> {
    all_dadjokes.order(dadjokes::id.desc()).load::<Dadjoke>(conn)
  }

  pub fn find_with_id(id: String, conn: &PgConnection) -> QueryResult<Dadjoke> {
    all_dadjokes.find(id).first(conn)
  }

  pub fn insert(dadjoke: NewDadjoke, conn: &PgConnection) -> QueryResult<Dadjoke> {
    diesel::insert_into(dadjokes::table)
      .values(&dadjoke)
      .get_result(conn)
  }

  pub fn update(dadjoke: Dadjoke, conn: &PgConnection) -> QueryResult<Dadjoke> {
    diesel::update(all_dadjokes.find(dadjoke.id))
      .set(dadjokes::text.eq(dadjoke.text))
      .get_result(conn)
  }

  pub fn delete_with_id(id: String, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(all_dadjokes.find(id)).execute(conn)
  }
}