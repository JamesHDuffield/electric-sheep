use diesel::prelude::*;
use crate::schema::categories;
use rand::seq::SliceRandom;

#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: i32,
    pub description: String,
}

impl Categories {
    pub fn select_random(connection: &mut PgConnection) -> Self {
        let categories = categories::dsl::categories.load::<Categories>(connection).expect("Issue retrieving categories");
        categories.choose(&mut rand::thread_rng()).expect("Issue selecting category").clone()
    }
}