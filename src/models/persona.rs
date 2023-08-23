use crate::schema::personas;
use diesel::prelude::*;
use rand::seq::SliceRandom;

#[derive(Queryable, Debug)]
#[diesel(table_name = personas)]
pub struct Persona {
    pub id: i32,
    pub text: String,
}

impl Persona {
    pub fn select_random_persona(connection: &mut PgConnection) -> String {
        let personas = personas::dsl::personas
            .load::<Self>(connection)
            .expect("Issue retrieving personas");
        let persona = personas
            .choose(&mut rand::thread_rng())
            .expect("Issue selecting persona");
        persona.text.clone()
    }
}
