use diesel::prelude::*;
use crate::schema::questions;
use super::Categories;
use rand::seq::SliceRandom;

#[derive(Queryable, Debug)]
#[diesel(table_name = questions)]
pub struct Question {
    pub id: i32,
    pub text: String,
    pub category_id: i32,
}

impl Question {
    pub fn select_random_questions_from_category(connection: &mut PgConnection, category: &Categories, amount: usize) -> Vec<String> {
        let questions: Vec<Question> = questions::dsl::questions.filter(questions::category_id.eq(category.id)).load::<Question>(connection).expect("Issue retrieving questions");
        questions.choose_multiple(&mut rand::thread_rng(), amount).map(|question| question.text.clone()).collect::<Vec<String>>()
    }
}