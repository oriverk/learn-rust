use std::collections::HashMap;

pub fn calculate_largest(numbers: &Vec<i32>) -> i32 {
    let mut largest = numbers[0];

    for &item in numbers.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

pub fn calculate_mean(numbers: &Vec<i32>) -> f64 {
    //! 平均値
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    sum as f64 / count as f64
}

/// 中央値
pub fn calculate_median(numbers: &Vec<i32>) -> f64 {
    let mut sorted_numbers: Vec<i32> = numbers.clone();
    sorted_numbers.sort();

    let count = sorted_numbers.len();
    if count % 2 == 0 {
        (sorted_numbers[count / 2 - 1] + sorted_numbers[count / 2]) as f64 / 2.0
    } else {
        sorted_numbers[count / 2] as f64
    }
}

pub fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    //! 最頻値
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn max(){
    let numbers: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    assert_eq!(7, calculate_largest(&numbers))
  }

  #[test]
  fn mean(){
    let numbers: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    assert_eq!(4.0, calculate_mean(&numbers))
  }

  #[test]
  fn median(){
    let numbers: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    assert_eq!(4.5, calculate_median(&numbers))
  }

  #[test]
  fn mode(){
    let numbers: Vec<i32> = vec![1, 2, 2, 3, 4, 5, 5, 5, 6, 7];
    assert_eq!(5, calculate_mode(&numbers))
  }
}
