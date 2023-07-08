use std::collections::HashMap;
use std::fmt;

use super::ingredient::Ingredient;
use super::{IngredientDb, Printable};

pub struct Recipe {
    pub name: String,
    pub total_calories: i32,
    pub ingredients: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct MissingIngErr;

impl fmt::Display for MissingIngErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing ingredient")
    }
}

impl Recipe {
    pub fn new(name: String) -> Recipe {
        Recipe {
            name,
            total_calories: 0,
            ingredients: HashMap::new(),
        }
    }

    pub fn add_ingredient(
        &mut self,
        ingredient: String,
        quantity: f32,
        db: &IngredientDb,
    ) -> Result<bool, MissingIngErr> {
        if db.ingredients.contains_key(&ingredient) {
            self.ingredients.insert(
                db.ingredients.get(&ingredient).unwrap().name.clone(),
                quantity,
            );
            Ok(true)
        } else {
            Err(MissingIngErr)
        }
    }
}

impl Printable for Recipe {
    fn print(&self) -> String {
        let mut db_string: String = String::new();
        db_string.push_str(format!("{:20} | {:5} \n", self.name, self.total_calories).as_str());

        for (ing, qty) in &self.ingredients {
            db_string.push_str(format!("-  {:20} | {:5}g", ing, qty).as_str());
            db_string.push_str("\n");
        }

        db_string
    }
}
