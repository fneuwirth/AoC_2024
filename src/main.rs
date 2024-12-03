pub mod day_01;

fn main() {
  let results = [day_01::day::task()];
  print_table(&results);
}

fn calc_max_widths(input: &[(i32, i32)]) -> (usize, usize) {
  let max_width_first_column = input
    .iter()
    .map(|(first, _)| first)
    .max()
    .map(|v| v.to_string())
    .unwrap_or_default()
    .len();

  let max_width_sec_column = input
    .iter()
    .map(|(_, sec)| sec)
    .max()
    .map(|v| v.to_string())
    .unwrap_or_default()
    .len();

  (max_width_first_column, max_width_sec_column)
}

fn print_table(results: &[(i32, i32)]) {
  let (max_width_first_column, max_width_sec_column) = calc_max_widths(results);

  // print table
  println!(
    "Day | {: ^max_width_first_column$} | {: ^max_width_sec_column$}",
    "Part 1", "Part 2"
  );
  println!(
    "----+-{:-^max_width_first_column$}-+-{:-^max_width_sec_column$}-",
    "", ""
  );
  results
    .iter()
    .enumerate()
    .for_each(|(idx, (first, second))| {
      println!(
        "{:0>2}  | {: >max_width_first_column$} | {: >max_width_sec_column$}",
        idx + 1,
        first,
        second
      );
    });
}
