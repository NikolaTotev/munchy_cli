pub struct Ingredient {
    pub name: String,
    pub calories_100g: f32,
}

impl Ingredient {
    fn new(name: String, calories: f32) -> Ingredient {
        Ingredient {
            name: name,
            calories_100g: calories,
        }
    }

    fn calories_per100(&self) -> f32 {
        self.calories_per100() / 100.0
    }
}
