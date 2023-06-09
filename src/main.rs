use std::io;
fn fahrenheit_to_celcius() {
  println!("\nWhat would you like to convert to celcius? >");

  let mut one = String::new();

  io::stdin().read_line(&mut one).expect("Failed to read line");

  let one: f64 = one.trim().parse::<f64>().expect("Failed to read line");

  let _conversion = (one - 32.0) * 5.0 / 9.0;

  println!("\n\n{one} degrees fahrenheit in celcius is {_conversion} degrees celcius\n\n");
}

fn celcius_to_fahrenheit() {
  println!("\n\nWhat would you like to convert to fahrenheit? >");

  let mut one2 = String::new();

  io::stdin().read_line(&mut one2).expect("Failed to read line");

  let one2: f64 = one2.trim().parse::<f64>().expect("Failed to read line");

  let _conversion2 = (one2 * 9.0/5.0) + 32.0;

  println!("\n\n{one2} degrees celcius is {_conversion2} degrees fahrenheit\n\n");
}

fn main() {
  repeated_input();
  println!("-------------------------------------------------------");
  repeated_input1();
  println!("Thank you for using my program!!\nTo check more if you want, you can check my github repositories at https://github.com/Swoiksdmop or even contribute to the projects.\n\nYou can email me at kigangadarell@gmail.com or ethankiganga@outlook.com.");
}

fn repeated_input() {
  for _ in 0..3 {
    fahrenheit_to_celcius();
  }
}

fn repeated_input1() {
    for _ in 0..3 {
    celcius_to_fahrenheit();
  }
}
