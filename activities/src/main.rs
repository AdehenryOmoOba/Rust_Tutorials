// Enums
// enum Color {
//     Red,
//     Blue,
//     Green,
// }

// fn print_color(color: Color) {
//     match color {
//         Color::Blue => println!("Up Chelsea, the color is Blue!"),
//         Color::Green => println!("Green!"),
//         _ => println!("Either Red or other color but not Blue or Green!"),
//     }
// }

// fn main() {
//     print_color(Color::Blue)
// }

// ===============================================================================

// Struct
// enum Flavor {
//     Sweet,
//     Sour,
//     Vanilla,
// }
// struct Drink {
//     flavor: Flavor,
//     weight: f64,
// }

// fn print_drink(drink: Drink) {
//     match drink.flavor {
//         Flavor::Sweet => println!("The falvor is sweet and weighs {}oz!!!", drink.weight),
//         _ => println!(
//             "The falvor is either sour or vanilla and weighs {}oz 😕",
//             drink.weight
//         ),
//     }
// }
// fn main() {
//     let sweet_drink = Drink {
//         flavor: Flavor::Sweet,
//         weight: 35.9,
//     };

//     let sour_drink = Drink {
//         flavor: Flavor::Sour,
//         weight: 100.07,
//     };

//     let vanilla_drink = Drink {
//         flavor: Flavor::Vanilla,
//         weight: 100.07,
//     };

//     print_drink(sweet_drink);
//     print_drink(sour_drink);
//     print_drink(vanilla_drink);
// }

// ==================================================

// Tuples
// fn generate_tuples() -> (i32, f64) {
//     return (100, 12.75);
// }
// fn main() {
//     // un-destructured tuples
//     let data = generate_tuples();
//     // Destructured tuples
//     let (age, height) = generate_tuples();

//     println!("Ade is {}years old and {}ft tall", data.0, data.1);
//     println!("Henry is {}years old and {}ft tall", age, height);
// }

// =========================================================

// impl
// struct Car {
//     name: String,
//     year: i32,
//     price: f64,
// }

// impl Car {
//     fn print_car_details(&self) {
//         println!("My car name: {:?}", self.name);
//         println!("My car price: ${:?}", self.price);
//         println!("My car production year: {:?}", self.year);
//     }
// }

// fn main() {
//     let my_car = Car {
//         name: "Honda".to_owned(),
//         price: 550.00,
//         year: 2023,
//     };

//     my_car.print_car_details()
// }

// =============================================

// Vectors
// fn main() {
//     let mut age_list = Vec::new();
//     let people = vec!["Buy Grocery", "View Cart", "Calculate Cart Total", "Quit"];

//     age_list.push(22);
//     age_list.push(41);
//     age_list.push(77);

//     for age in &age_list {
//         println!("Age: {:?}", age);
//     }

//     let mut list_number = 0;

//     for action in &people {
//         list_number += 1;
//         println!("{:?}. {:?}", list_number, action);
//     }
// }

// ===============================================

// strings
fn print_name(first_name: &str, last_name: &str) {
    println!("Full name: {first_name} - {last_name}");
}
fn main() {
    // let first_name = "Ade";
    // let last_name = "Henry";
    print_name("Ade", "Henry")
}
