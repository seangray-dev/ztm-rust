- Match Expressions

  - add logic to program
  - similar to if..else
  - Exhaustive
    - all options must be accounted for

  ```rust
  fn main() {
  	let some_bool = true;
  	match some_bool {
  		true => prtinln!("its true");
  		false => println!("its false");
  	}
  }

  fn main() {
  	let some_int = 3;
  	match some_int {
  		1 => println!("its 1");
  		2 => println!("its 2");
  		3 => println!("its 3");
  		_ => ptintln!("its something else");
  	}
  }
  ```

  match vs else…if

  - match will be checked by the compiler
    - if a new possibility is added, you will be notified when this occurs
  - match considers all possibilities
    - more robust code
  - Use underscore `(_)` to match ‘anything else’
