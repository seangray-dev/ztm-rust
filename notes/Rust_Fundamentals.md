# Rust Fundamentals

## Data Types

- Memory only stores binary data
  - anything can be represented in binary
- Program determines what the binary represents
- Basic types that universally useful are provided by the language
- Boolean
  - `true` or `false`
- Integer
  - `1`, `2`, `50`, `99`, `-2`
- Double / Float
  - `1.1`, `5.5`, `200.0001`, `2.0`
- Character
  - `'A'`, `'B'`, `'6'`, `'$'`
- String
  - `"strings"`, `"use"`, `"double"`, `"quotes"`

## Variables

- Assings data to a temporary memory location
  - Allows programmer to easily work with memory
- Can be set to any value & type
- Immutable by default, but can be mutuable
- Examples:

```rust
let two = 2;
let hello = "hello";
let j = 'j';
let my_half = 0.5;
let mut my_name = "Bill";
let quit_program = false;
let your_half = my_half;
```

## Functions

What are functions?

- A way to encapsulate program functionality
- Optionally accept data
- Optionally return data
- Utilized for code organization, also makes code easier to read

### Anatomy of a function:

```rust
fn add(a: i32, b: i32) -> i32 {
	a + b
}

// fn begins a new function
// (add) name of the function, can be anything but can't start with a number or other keyword
// function paramters (a, b) followed by a colon and the name of the type
// create and arrow and specify the return type
// function body between the curly braces

// Using a function

let x = add(1, 1); // Output: 2
let y = add(3, 0); // Output: 3
let z = add(x, 1); // Output: 3
```

## Print line macro

- Macros expand into additional code
- println "prints" information to the console
- Useful for debugging

### Example:

```rust
let life = 42;
println!("hello");
println!("{:?}", life);
```

- Macros use an exclamation point to call/invoke
- Generate additional Rust code
- Data can be printed using `println!:`
  - `{:?}`
  - `{varname:?}`

## Control Flow with `if`

- Code is executed line by line
- Actions are performed & control flow may change
- Specifc conditions can change control flow
  - `if` statements
  - `else` statements
  - `else if` statements

### Example:

```rust

let a = 99;
if a > 99 {
	println!("Big number");
} else {
	println!("Small number");
}

// Nested if..else

let a = 99;
if a > 99 {
	if a > 200 {
		println!("Huge number");
	} else {
		println!("Big number");
	}
} else {
	println!("Small number");
}
```

## Repition

- Called 'looping' or 'iteration'
- Multiple types of loops
- `loop` - infinite loop
- `while` - conditional loop
- Both types of loops can exit using the `break` command

### Example:

```rust
// loop

let mut a = 0;
loop {
	if a == 5 {
		break;
	}
	println!("{:?}", a);
	a = a + 1;
}

// while loop

let mut a = 0;
while a != 5 {
	println!("{:?}", a);
	a = a + 1;
}
```

## Numeric Types & Basic Arithmetic

```rust
let sum = 2 + 2; // 4
let value = 10 - 5; // 5
let division = 10 / 2; // 5
let mult = 5 * 5; // 25
let rem = 6 % 3; // 0
let rem = 6 % 4; // 2

fn subtract(a: i32, b: i32) -> i32 {
	a - b
}
```
