// 1. мій перший проєкт на расті, в цьому акаунті джіт хаба
fn celsius_to_fahrenheit(c: f64) -> f64 {

(c * 1.8) + 32.0

}

fn fahrenheit_to_celsius(f: f64) -> f64 {

(f - 32.0) / 1.8

}

fn main() {


let test_celsius = 25.0;


  let fahrenheit = celsius_to_fahrenheit(test_celsius);
  println!("{}°C — це {}°F", test_celsius, fahrenheit);

  let test_fahrenheit = 77.0;
  let celsius = fahrenheit_to_celsius(test_fahrenheit);
  println!("{}°F — це {}°C", test_fahrenheit, celsius);

    
}