use crate::ingredient_range::IngredientRange;
use std::collections::HashMap;
use uuid::Uuid;

pub struct IngredientsDB {
  pub ingredient_ids: HashMap<Uuid, IngredientRange>,
}

impl IngredientsDB {
  pub fn new() -> IngredientsDB {
    IngredientsDB {
      ingredient_ids: HashMap::new(),
    }
  }

  pub fn add_ingredient_id_range(&mut self, start: i64, end: i64) {
    let mut expanded = false;
    let ingredient = IngredientRange::new(start, end);
    for range in self.ingredient_ids.values_mut() {
      if range.is_completely_internal(&ingredient) {
        // this range is already covered
        return;
      }

      expanded = range.expand(start, end);
      if expanded {
        // this range has expanded an existing range
        return;
      }
    }
    if !expanded {  
      self.ingredient_ids.insert(ingredient.id, ingredient);
    }
  }

  pub fn merge_ranges(&mut self) -> bool {
    let mut ranges = self.ingredient_ids.values().cloned().collect::<Vec<IngredientRange>>();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut remove_range_idxes:Vec<usize> = Vec::new();
    let max = ranges.len();
    let mut did_merge = false;
    let mut last_expanded = true;
    let mut current_range_idx:usize = 0;

    for idx1 in 0..max-1 {
      if !last_expanded {
        current_range_idx = idx1;
      }
      let range2 = ranges[idx1+1];
      last_expanded = ranges[current_range_idx].expand(range2.start, range2.end);

      if last_expanded {
        remove_range_idxes.push(idx1+1);
      }
      did_merge = did_merge || last_expanded;
    }


    for idx in remove_range_idxes.into_iter().rev() {
      ranges.remove(idx);
    }
    self.ingredient_ids.clear();

    self.ingredient_ids = ranges.into_iter().map(|r| (r.id, r)).collect();

    did_merge
  }

  pub fn get_range_count(&self) -> usize {
    self.ingredient_ids.len()
  }

  pub fn total_nums_in_ranges(&self) -> i64 {
    let mut total: i64 = 0;
    for range in self.ingredient_ids.values().into_iter() {
      let size = range.size();
      total += size;
    }
    total
  }

  #[allow(dead_code)]
  pub fn print_ranges(&self) {
    println!("-----");
    for range in self.ingredient_ids.values().into_iter() {
      println!("Range {} {} - Size {}", range.start, range.end, range.size());
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_new_db() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(102, 150);
    assert_eq!(db.get_range_count(), 2);
  }

  #[test]
  fn test_add_range_internal() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(30, 50);
    assert_eq!(db.get_range_count(), 1);
    assert_eq!(db.total_nums_in_ranges(), 81);
  }

  #[test]
  fn test_add_range_lower() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(15, 50);
    assert_eq!(db.get_range_count(), 1);
    assert_eq!(db.total_nums_in_ranges(), 86);
  }

  #[test]
  fn test_add_range_higher() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(90, 105);
    assert_eq!(db.get_range_count(), 1);
    assert_eq!(db.total_nums_in_ranges(), 86);
  }

  #[test]
  fn test_add_range_triple() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 50);
    db.add_ingredient_id_range(95, 105);
    db.add_ingredient_id_range(45, 100);
    db.print_ranges();
    assert_eq!(db.get_range_count(), 2);
    db.merge_ranges();
    assert_eq!(db.get_range_count(), 1);
    assert_eq!(db.total_nums_in_ranges(), 86);
  }

  #[test]
  fn test_add_range_concurrent() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 50);
    db.add_ingredient_id_range(51, 105);
    assert_eq!(db.get_range_count(), 2);
    db.merge_ranges();
    assert_eq!(db.get_range_count(), 2);
    assert_eq!(db.total_nums_in_ranges(), 86);
  }

  #[test]
  fn test_merge() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(120, 150);
    db.add_ingredient_id_range(95, 125);
    let did_merge = db.merge_ranges();
    assert!(did_merge);
    assert_eq!(db.get_range_count(), 1);
    assert_eq!(db.total_nums_in_ranges(), 131);
  }

  #[test]
  fn test_merge_internal() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(30, 50);
    let did_merge = db.merge_ranges();
    assert!(!did_merge);
    assert_eq!(db.get_range_count(), 1);
    assert_eq!(db.total_nums_in_ranges(), 81);
  }

  #[test]
  fn test_merge_separate() {
    let mut db = crate::IngredientsDB::new();
    db.add_ingredient_id_range(20, 100);
    db.add_ingredient_id_range(10, 15);
    let did_merge = db.merge_ranges();
    assert!(!did_merge);
    assert_eq!(db.get_range_count(), 2);
    assert_eq!(db.total_nums_in_ranges(), 87);
  }
}