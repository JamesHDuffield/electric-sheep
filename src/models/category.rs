use diesel::prelude::*;

#[derive(Queryable, Debug)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: i32,
    pub description: String,
}