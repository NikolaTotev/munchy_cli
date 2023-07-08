pub struct Ingredient {
    pub name: String,
    pub calories_100g: f32,
}

impl Ingredient {
   pub fn new(name: String, calories: f32) -> Ingredient {
        Ingredient {
            name: name,
            calories_100g: calories,
        }
    }

    pub fn calories_per1(&self) -> f32 {
        self.calories_100g / 100.0
    }
}
