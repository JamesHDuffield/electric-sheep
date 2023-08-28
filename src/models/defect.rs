use super::Categories;
use crate::schema::defects;
use diesel::prelude::*;
use diesel::sql_types::Integer;

#[derive(Queryable, Debug, QueryableByName)]
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
        let defect = diesel::sql_query("SELECT * FROM defects WHERE category_id = $1 OFFSET floor(random() * (SELECT count(1) from defects WHERE category_id = $1)) LIMIT 1")
            .bind::<Integer, _>(category.id)
            .get_result::<Self>(connection)?;
        Ok(defect.text.to_owned())
    }
}
