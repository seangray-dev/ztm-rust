# Working With Data

## Enumeration

- Data that can be one of multiple different possibilities
  - Each possibility is called a 'variant'
  - enums can only be one variant at a time
- Provides information about your program to the compiler
  - More robust programs

### Example:

```rust
enum Direction {
  Up,
  Down,
  Left,
  Right
}

fn which_way (go: Direction) {
  match go {
    Direction::Up => println!("Up"),
    Direction::Down => println!("Down"),
    Direction::Left => println!("Left"),
    Direction::Right => println!("Right")
  }
}
```

## Structure

- A type that contains multiple pieces of data
  - All or nothing - cannot have some pieces of data and not others
- Each piee of data is called a 'field'
- Makes working with data easier
  - Similar data can be grouped together
- All fields must be present to create a struct
- Fields can be accessed using the `.`

### Example:

```rust
struct ShippingBox {
  depth: i32;
  width: i32;
  height i32;
}

let my_box = ShippingBox {
  depth: 10,
  width: 10,
  height: 10
}

let tall = my_box.height;
println!("the box is {:?} units tall", tall);
```

## Tuples

- A type of 'record'
- Store data anonymously
  - No field names
- Useful to return pairs of data from functions
- Can be 'de-structured' easily into variables
- Can contain any number of fields
  - use struct when more than 2-3 fields

### Example:

```rust
enum Access {
	Full,
}

fn one_two_three() -> (i32, i32, i32) {
	(1,2,3)
}

let numbers = one_two_three();
let (x, y, z) = one_two_three();
prtinln("{:?}, {:?}", x, numbers.0); // 1
prtinln("{:?}, {:?}", x, numbers.1); // 2
prtinln("{:?}, {:?}", x, numbers.2); // 3

let (employee, access) = ("Jake", Access::Full);

```

## Expressions

- Rust is an expression-based language
  - Most things are evaluated and return some value
- Epxression values coalesce to a single point
  - Can be used for nesting logic
    - `if` and `match` expressions can be nested but best to not use more than 2 or 3 levels.

### Examples:

```rust
// 1.
let my_num = 3;
let is_lt_5 = if my_num < 5 {
  true
} else {
  false
};

// 2.
let my_num = 3;
let message = match my_num {
  1 => "hello",
  _ => "goodbye",
}

// 3.
enum Menu {
  Burger,
  Fries,
  Drink,
}

let paid = true;
let item = Menu::Drink;
let drink_type = "water";
let order_placed = match item {
  Menu::Drink => {
    if drink_type == "water" {
      true
    } else {
      false
    }
  }
  _ => true
}
```
