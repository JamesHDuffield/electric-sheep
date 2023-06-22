use diesel::prelude::*;
use crate::schema::categories;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: i32,
    pub description: String,
}