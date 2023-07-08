use std::io;

use munchy_cli::munchy::core::ingredient;
use munchy_cli::munchy::core::ingredient::*;
use munchy_cli::munchy::core::recipe::*;
use munchy_cli::munchy::core::CookBook;
use munchy_cli::munchy::core::{self, IngredientDb};

fn main() {
    let db: IngredientDb = IngredientDb::new("Basics".to_string());
    createRecipe(&db);
}

fn createRecipe(ingrDb: &IngredientDb) {
    let mut name = String::new();
    println!("Enter recipe name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let mut new_recipe: Recipe = Recipe::new(name);

    let mut ingredient_name = String::new();
    let mut ingredient_qty = String::new();

    loop {
        print!("Ingredient name:");
        io::stdin()
            .read_line(&mut ingredient_name)
            .expect("Failed to read line");

        print!("Quantity:");
        io::stdin()
            .read_line(&mut ingredient_qty)
            .expect("Failed to read line");

        if ingredient_name == "done" {
            break;
        }

        let ingredient_qty: f32 = match ingredient_qty.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong ingredient quantity.");
                continue;
            }
        };

        match new_recipe.add_ingredient(ingredient_name.clone(), ingredient_qty, ingrDb) {
            Ok(_) => print!(""),
            Err(MissingIngErr) => print!("No such ingredient. Given ingredient not added."),
        }
    }
}

fn add_recipe_to_cookbook(recipe: Recipe, cook_book: CookBook) {}

fn createIngredient() {}

fn add_ingredient_to_db(ingr: Ingredient, db: IngredientDb) {}

//Step 1
/*

* Model a recipe, food ingredient, cook book and ingredient database. - DONE
* Implement creation/instantiation for each of the items mentioned above. - DONE
* Implement adding a food ingredient by name and by calories/100g - DONE
* Implement creating a recipe by adding a name, food ingredient with quantities. - DONE
  Validate the ingredient is actually in the igredient db. - DONE
* Implement adding a recipe to the cookbook - DONE
* Implement listing of ingredients in the database and cookbook - DONE
* Implement searching recipe by name - DONE
* Implement single recipe view - DONE

*/

/*
 * Implement UI
*/

//Step 2
/*

* Implement basic search by name
* Implement search that takes in an ingredient and returnc recipes that have at least one of the ingredients
* Implement saving to json for the db of ingredients and the cookbook
* Implement opnening json for ingredient db and cookbook

*/

//Step 3
/*

* Improve printing to console.
* Improve user interface

*/
