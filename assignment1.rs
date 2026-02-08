// Temperature Converter

const FREEZING_FAHRENHEIT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_FAHRENHEIT) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_FAHRENHEIT
}

fn main() {
    let mut temp_f: f64 = 68.0;      //test fahrenheit_to_celsius

    println!("{} °F = {:.2} °C", temp_f, fahrenheit_to_celsius(temp_f));

    let mut i = 1;
    while i <= 5 {                  //print next 5 intergers
        temp_f = temp_f + 1.0;
        println!("{} °F = {:.2} °C", temp_f, fahrenheit_to_celsius(temp_f));
        i = i + 1; 
    }
}