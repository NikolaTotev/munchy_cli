use munchy_cli::munchy::core::ingredient::*;
use munchy_cli::munchy::core::recipe::*;
use munchy_cli::munchy::core::CookBook;
use munchy_cli::munchy::core::IngredientDb;
use std::io;

fn main() {
    let mut db: IngredientDb = IngredientDb::new("Basics".to_string());
    create_ingredient(&mut db);
    create_ingredient(&mut db);

    createRecipe(&db);
}

fn createRecipe(ingrDb: &IngredientDb) -> Recipe {
    let mut name = String::new();
    println!("Enter recipe name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let mut new_recipe: Recipe = Recipe::new(name);

    let mut ingredient_name = String::new();
    let mut ingredient_qty = String::new();

    loop {
        println!("Ingredient name:");
        io::stdin()
            .read_line(&mut ingredient_name)
            .expect("Failed to read line");

          println!("{ingredient_name}");
        
        if ingredient_name.trim() == "done" {
            break;
        }

        println!("Quantity (grams):");
        io::stdin()
            .read_line(&mut ingredient_qty)
            .expect("Failed to read line");

        let qty: f32 = match ingredient_qty.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong ingredient quantity.");
                continue;
            }
        };

        match new_recipe.add_ingredient(ingredient_name.clone(), qty, ingrDb) {
            Ok(_) => {
              println!("Added {ingredient_qty}g of {ingredient_name}");
              ingredient_name.clear();              
              ingredient_qty.clear();
            },
            Err(MissingIngErr) => println!("No such ingredient. Given ingredient not added."),
        }
    }
    println!("Recipe created!");
    new_recipe      
}

fn add_recipe_to_cookbook(_recipe: Recipe, _cook_book: CookBook) {}

fn create_ingredient(ing_db: &mut IngredientDb) {
    println!("Ingredient name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter calories per 100g:");

    loop {
        let mut calories = String::new();
        io::stdin()
            .read_line(&mut calories)
            .expect("Failed to read line");
        match calories.trim().parse() {
            Ok(num) => {
                let new_ingredient: Ingredient = Ingredient::new(name, num);
                ing_db.addIngredient(new_ingredient);
                break;
            }
            Err(_) => {
                println!("Calorie must be a valid number.");
                continue;
            }
        };
    }
}

fn add_ingredient_to_db(_ingr: Ingredient, _db: IngredientDb) {}

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
