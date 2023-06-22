use diesel::prelude::*;
use crate::schema::defects;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = defects)]
pub struct Defect {
    pub id: i32,
    pub text: String,
    pub category_id: i32,
}