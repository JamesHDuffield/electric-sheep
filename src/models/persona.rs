use crate::schema::personas;
use diesel::prelude::*;

#[derive(Queryable, Debug, QueryableByName)]
#[diesel(table_name = personas)]
pub struct Persona {
    pub id: i32,
    pub text: String,
}

impl Persona {
    pub fn select_random_persona(connection: &mut PgConnection) -> QueryResult<String> {
        let persona = diesel::sql_query("SELECT * FROM personas OFFSET floor(random() * (SELECT count(1) from personas)) LIMIT 1")
            .get_result::<Self>(connection)?;
        Ok(persona.text.to_owned())
    }
}
