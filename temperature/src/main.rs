
use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");
    loop{
        show_menu();
        let choice =get_choices();

       match choice {
            1=>  convert_temperature(),
            2=>show_conversion_table(),
            3=>{
               println!("Thank you for using the Temperature Converter!");
               break;
            },
            _=>{
               println!("Invalid choice. Please try again.");       
            }

       } 
    }
}




fn get_choices()->u32{
   let  mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0,
    }
}

fn show_conversion_table(){
   println!("Celsius\tFahrenheit\tKelvin");
   println!("-------\t----------\t-------");
    println!("water boils at 100 degrees celsius");
    println!("ice melts at -273 degrees celsius");
}


fn show_menu() {
    println!("=== Temperature Converter Menu ===");
    println!("1. Convert Temperature");
    println!("2. Show Conversion table");
    println!("3. Exit!!");
    
}

fn convert_temperature()  {
    println!("Enter the temperature");
    let mut temp = String::new();
     io::stdin().read_line(&mut temp).expect("error happened");     
      let temp = temp.trim().parse::<f64>().expect("error happened");

     println!("Enter the scale your temperature is K/C/F") ;
    let mut scale =String::new();
    io::stdin().read_line(&mut scale).expect("error happened");
     let scale = scale.trim().to_uppercase();

   match scale.as_str() {
       "C" => {
            let fahrenheit = celsius_to_fahrenheit(temp);
            let kelvin = celsius_to_kelvin(temp);
            println!("{:.2}°C = {:.2}°F = {:.2}K", temp, fahrenheit, kelvin);
        },
        "F" => {
            let celsius = fahrenheit_to_celsius(temp);
            let kelvin = celsius_to_kelvin(celsius);
            println!("{:.2}°F = {:.2}°C = {:.2}K", temp, celsius, kelvin);
        },
        "K" => {
            let celsius = kelvin_to_celsius(temp);
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{:.2}K = {:.2}°C = {:.2}°F", temp, celsius, fahrenheit);
        },
        _=>{
            println!("Invalid scale. Please try again.");               

      }
   }
}



// TODO: Implement these conversion functions
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
   let f = celsius * 9.0 / 5.0 + 32.0;
   f

}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    // Your implementation here
    let k = celsius + 273.15;
    k
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // Your implementation here
    let c = (fahrenheit - 32.0) * 5.0 / 9.0;
    c
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    // Your implementation here
    let c = kelvin - 273.15;
    c
}