use crate::schema::categories;
use diesel::{prelude::*, sql_types::Text};

#[derive(Queryable, QueryableByName, Clone, Debug)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: i32,
    pub description: String,
}

impl Categories {
    pub fn select_random(connection: &mut PgConnection) -> QueryResult<Self> {
        let category = diesel::sql_query("SELECT * FROM categories OFFSET floor(random() * (SELECT count(1) from categories)) LIMIT 1")
            .get_result::<Self>(connection)?;
        Ok(category.clone())
    }

    pub fn select_by_description(connection: &mut PgConnection, description: &String) -> QueryResult<Self> {
        let category = diesel::sql_query("SELECT * FROM categories WHERE description = $1 LIMIT 1")
            .bind::<Text, _>(description)
            .get_result::<Self>(connection)?;
        Ok(category.clone())
    }
}
