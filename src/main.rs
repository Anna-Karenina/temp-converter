use dialoguer::{theme::ColorfulTheme, Select};
use std::io;
static HEAT_UNITS: &'static [&str] = &["Farengate", "Celsius"];

fn main() {
  let mut input_value = String::new();
  let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Pick your heat unit")
    .default(0)
    .items(&HEAT_UNITS[..])
    .interact()
    .unwrap();

  println!("Selected heat unit is: {}!", HEAT_UNITS[selection]);

  io::stdin()
    .read_line(&mut input_value)
    .expect("some message");

  let input_value: f64 = input_value.trim().parse().expect("need a number");

  if HEAT_UNITS[selection] == HEAT_UNITS[0] {
    let temp = convert_to_faringate(input_value).round();
    println!("converted {}", temp)
  } else {
    let temp = convert_to_celsius(input_value).round();
    println!("converted {}", temp)
  }
}
fn convert_to_celsius(temp_f: f64) -> f64 {
  let num = 5.0 / 9.0;
  (temp_f - 32.0) * num
}

fn convert_to_faringate(temp_c: f64) -> f64 {
  let num = 9.0 / 5.0;
  (temp_c * num) + 32.0
}
