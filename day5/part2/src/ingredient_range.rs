use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IngredientRange {
  pub id: Uuid,
  pub start: i64,
  pub end: i64,
}

impl IngredientRange {
  pub fn new(start: i64, end: i64) -> IngredientRange {
    IngredientRange { id: Uuid::new_v4(), start, end }
  }

  pub fn is_completely_internal(&self, other: &IngredientRange) -> bool {
    return 
      other.start >= self.start && 
      other.start <= self.end && 
      other.end <= self.end && 
      other.end >= self.start;
  }

  pub fn expand(&mut self, start: i64, end: i64) -> bool {
    let mut has_expanded_start = false;
    let mut has_expanded_end = false;
    let original_start = self.start;
    let original_end = self.end;
    
    if start < original_start && !(end < original_start)  {
      self.start = start;
      has_expanded_start = true;
    }

    if end > original_end && !(start > original_end) {
      self.end = end;
      has_expanded_end = true;
    }
    return has_expanded_start || has_expanded_end;
  }

  #[allow(dead_code)]
  pub fn contains(&self, num: i64) -> bool {
    return num >= self.start && num <= self.end;
  }

  pub fn size(&self) -> i64 {
    return self.end - self.start + 1;
  }
}

#[cfg(test)]
mod tests {
  use crate::ingredient_range::IngredientRange;

  #[test]
  fn test_new() {
    let range = crate::ingredient_range::IngredientRange::new(20, 100);
    assert_eq!(range.start, 20);
    assert_eq!(range.end, 100);
  }

  #[test]
  fn test_internal() {
    let range = crate::ingredient_range::IngredientRange::new(20, 100);
    assert!(range.is_completely_internal(&IngredientRange::new(30, 35)));
    assert!(range.is_completely_internal(&IngredientRange::new(20, 25)));
    assert!(range.is_completely_internal(&IngredientRange::new(95, 100)));
    assert!(!range.is_completely_internal(&IngredientRange::new(90, 101)));
    assert!(!range.is_completely_internal(&IngredientRange::new(15, 25)));
    assert!(range.is_completely_internal(&IngredientRange::new(20, 100)));
  }

  #[test]
  fn test_expand_noop_internal() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(25, 95);
    assert!(!expanded);
    assert_eq!(range.start, 20);
    assert_eq!(range.end, 100);
  }

  #[test]
  fn test_expand_noop_external() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(150, 175);
    assert!(!expanded);
    assert_eq!(range.start, 20);
    assert_eq!(range.end, 100);
  }

  #[test]
  fn test_expand_start() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(15, 95);
    assert!(expanded);
    assert_eq!(range.start, 15);
    assert_eq!(range.end, 100);
  }

  #[test]
  fn test_expand_start_limit() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(15, 20);
    assert!(expanded);
    assert_eq!(range.start, 15);
    assert_eq!(range.end, 100);
  }

  #[test]
  fn test_expand_end() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(25, 105);
    assert!(expanded);
    assert_eq!(range.start, 20);
    assert_eq!(range.end, 105);
  }

  #[test]
  fn test_expand_end_limit() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(100, 105);
    assert!(expanded);
    assert_eq!(range.start, 20);
    assert_eq!(range.end, 105);
  }

  #[test]
  fn test_expand_both() {
    let mut range = crate::ingredient_range::IngredientRange::new(20, 100);
    let expanded = range.expand(15, 105);
    assert!(expanded);
    assert_eq!(range.start, 15);
    assert_eq!(range.end, 105);
  }

  #[test]
  fn test_size() {
    let range = crate::ingredient_range::IngredientRange::new(20, 100);
    assert_eq!(range.size(), 81);
  }

  #[test]
  fn test_size_2() {
    let range = crate::ingredient_range::IngredientRange::new(3, 5);
    let range2 = crate::ingredient_range::IngredientRange::new(10, 20);
    assert_eq!(range.size() + range2.size(), 14);
  }
}