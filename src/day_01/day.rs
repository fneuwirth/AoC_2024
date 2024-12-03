use std::{collections::HashMap, fs, process::exit};

pub fn task() -> (i32, i32) {
  (task_one(), task_two())
}

fn task_one() -> i32 {
  let (mut left, mut right) = read_input();

  left.sort();
  right.sort();

  let mut results = vec![];

  for i in 0..left.len() {
    results.push((left[i] - right[i]).abs());
  }

  results.into_iter().sum::<i32>()
}

fn task_two() -> i32 {
  let (left, right) = read_input();
  let mut occurrence: HashMap<i32, i32> = HashMap::new();

  left.iter().for_each(|val| {
    if !occurrence.contains_key(val) {
      occurrence.insert(*val, right.iter().filter(|&cur| cur == val).count() as i32);
    }
  });

  let result = left.iter().map(|val| val * occurrence.get(val).unwrap());

  result.sum::<i32>()
}

fn read_input() -> (Vec<i32>, Vec<i32>) {
  let input_path = std::env::current_dir()
    .unwrap()
    .join("src")
    .join("day_01")
    .join("input")
    .join("input.txt");

  let raw_input = fs::read_to_string(input_path);
  if let Err(e) = raw_input {
    println!("{e}");
    exit(1);
  }

  let input = raw_input.unwrap();
  let input_list: Vec<_> = input.trim().split('\n').map(|f| f.to_owned()).collect();

  let mut left: Vec<i32> = vec![];
  let mut right: Vec<i32> = vec![];

  input_list.into_iter().for_each(|line| {
    let parts: Vec<_> = line.split("   ").collect();
    left.push(parts[0].parse().unwrap());
    right.push(parts[1].parse().unwrap());
  });

  (left, right)
}
