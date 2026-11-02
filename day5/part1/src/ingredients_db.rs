struct IngredientRange {
  pub start: i64,
  pub end: i64,
}

pub struct IngredientsDB {
  pub ingredients_ids: Vec<IngredientRange>,
}

impl IngredientsDB {
  pub fn new() -> IngredientsDB {
    IngredientsDB {
      ingredients_ids: Vec::new(),
    }
  }

  pub fn add_ingredient_id_range(&mut self, start: i64, end: i64) {
    let ingredient = IngredientRange { start, end };
    self.ingredients_ids.push(ingredient);
  }

  pub fn is_ingredient_id_valid(&self, id: i64) -> bool {
    for ingredient in &self.ingredients_ids {
      if id >= ingredient.start && id <= ingredient.end {
        return true;
      }
    }
    false
  }

  pub fn get_range_count(&self) -> usize {
    return self.ingredients_ids.len();
  }
}