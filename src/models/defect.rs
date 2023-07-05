use diesel::prelude::*;
use crate::schema::{defects, categories};
use super::Categories;
use rand::seq::SliceRandom;

#[derive(Queryable, Debug)]
#[diesel(table_name = defects)]
pub struct Defect {
    pub id: i32,
    pub text: String,
    pub category_id: i32,
}

impl Defect {
    pub fn select_random_defect(connection: &mut PgConnection) -> String {
        let categories = categories::dsl::categories.load::<Categories>(connection).expect("Issue retrieving categories");
        let category = categories.choose(&mut rand::thread_rng()).expect("Issue selecting category");
        let defects = defects::dsl::defects.filter(defects::category_id.eq(category.id)).load::<Defect>(connection).expect("Issue retrieving defects");
        let defect = defects.choose(&mut rand::thread_rng()).expect("Issue selecting defect");
        defect.text.clone()
    }
}