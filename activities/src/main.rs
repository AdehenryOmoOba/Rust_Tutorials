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
fn generate_tuples() -> (i32, f64) {
    return (100, 12.75);
}
fn main() {
    // un-destructured tuples
    let data = generate_tuples();
    // Destructured tuples
    let (age, height) = generate_tuples();

    println!("Ade is {}years old and {}ft tall", data.0, data.1);
    println!("Henry is {}years old and {}ft tall", age, height);
}
