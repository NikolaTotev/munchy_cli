use self::ingredient::*;
use self::recipe::*;
use std::collections::HashMap;
use std::hash::Hash;

pub mod ingredient;
pub mod recipe;

pub struct CookBook {
    name: String,
    recipes: HashMap<String, Recipe>,
}

impl CookBook {
    pub fn new(name: String) -> CookBook {
        CookBook {
            name,
            recipes: HashMap::new(),
        }
    }

    pub fn addRecipe(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.name.clone(), recipe);
    }

    pub fn get_recipe(&self, name: String) -> Option<&Recipe> {
        self.recipes.get(&name)
    }
}

impl Printable for CookBook {
    fn print(&self) -> String {
        let mut db_string: String = String::new();
        for (reci_name, reci) in &self.recipes {
            db_string.push_str(format!("{:20} | {:5} \n", reci.name, reci.total_calories).as_str());

            for (ing, qty) in &reci.ingredients {
                db_string.push_str(format!("-  {:20} | {:5}g", ing, qty).as_str());
            }
            db_string.push_str("\n");
        }
        db_string
    }
}

pub struct IngredientDb {
    name: String,
    ingredients: HashMap<String, Ingredient>,
}

impl IngredientDb {
    pub fn new(name: String) -> IngredientDb {
        IngredientDb {
            name,
            ingredients: HashMap::new(),
        }
    }

    pub fn addIngredient(&mut self, ingredient: Ingredient) {
        self.ingredients.insert(ingredient.name.clone(), ingredient);
    }
}

impl Printable for IngredientDb {
    fn print(&self) -> String {
        let mut db_string: String = String::new();
        for ingr in self.ingredients.values() {
            db_string.push_str(format!("{:20} | {:5} \n", ingr.name, ingr.calories_100g).as_str());
        }
        db_string
    }
}

pub trait Printable {
    fn print(&self) -> String;
}
