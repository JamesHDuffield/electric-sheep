use super::Categories;
use crate::schema::defects;
use diesel::prelude::*;
use rand::seq::SliceRandom;

#[derive(Queryable, Debug)]
#[diesel(table_name = defects)]
pub struct Defect {
    pub id: i32,
    pub text: String,
    pub category_id: i32,
}

impl Defect {
    pub fn select_random_from_category(
        connection: &mut PgConnection,
        category: &Categories,
    ) -> QueryResult<String> {
        let defects = defects::dsl::defects
            .filter(defects::category_id.eq(category.id))
            .load::<Defect>(connection)?;
        let defect = defects
            .choose(&mut rand::thread_rng())
            .ok_or(diesel::result::Error::NotFound)?;
        Ok(defect.text.clone())
    }
}
