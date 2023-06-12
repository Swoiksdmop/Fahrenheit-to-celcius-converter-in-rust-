use std::io;
fn main() {
  println!("Welcome to Ethan's program of converting Fahrenheit to Celcius!!!\n\nWhat would you like to convert to celcius? >");

  let mut one = String::new();

  io::stdin().read_line(&mut one).expect("Failed to read line");

  let one: f64 = one.trim().parse::<f64>().expect("Failed to read line");
  
  let _conversion = (one - 32.0) * 5.0 / 9.0;
  /*
  let _conversion2 = _conversion * 5.0;
  let _conversion3 = _conversion2 / 9.0;
*/
  println!("\n\n{one} degrees fahrenheit in celcius is {_conversion} degrees celcius");
}