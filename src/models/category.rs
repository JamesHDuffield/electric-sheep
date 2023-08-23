use crate::schema::categories;
use diesel::prelude::*;
use rand::seq::SliceRandom;

#[derive(Queryable, Clone, Debug)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: i32,
    pub description: String,
}

impl Categories {
    pub fn select_random(connection: &mut PgConnection) -> QueryResult<Self> {
        let categories = categories::dsl::categories.load::<Categories>(connection)?;
        let chosen = categories
            .choose(&mut rand::thread_rng())
            .ok_or(diesel::result::Error::NotFound)?;
        Ok(chosen.clone())
    }
}
