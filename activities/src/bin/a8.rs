// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
  Coke,
  Pepsi,
  MountainDew,
}

struct Drink {
  flavor: Flavor,
  fluid_ounces: f64,
}

fn print_drink(drink: Drink) {
  match drink.flavor {
    Flavor::Coke => println!("Coke"),
    Flavor::Pepsi => println!("Pepsi"),
    Flavor::MountainDew => println!("Mountain Dew"),
  }
  println!("{} ounces", drink.fluid_ounces);
}

fn main() {
  let coke = Drink {
    flavor: Flavor::Coke,
    fluid_ounces: 12.0,
  };

  let pepsi = Drink {
    flavor: Flavor::Pepsi,
    fluid_ounces: 12.0,
  };
  
  let mountain_dew = Drink {
    flavor: Flavor::MountainDew,
    fluid_ounces: 12.0,
  };

  print_drink(coke);
  print_drink(pepsi);
  print_drink(mountain_dew);
}
