/*  -----------------------------------------------------
    Highest Stock Price
        Description
        - We have weekwise stock prices. we want to retrieve
          the highest stock price in any week in little to no time.

        Tools
        - Maxstacks
        - Structures
        - Vectors
    -----------------------------------------------------
*/

struct MaxStack {
    main_stack: Vec<i32>,
    maximum_stack: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            main_stack: Vec::new(),
            maximum_stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.main_stack.push(value);
        /*
            Since `self.maximum_stack.last().unwrap()` returns a reference value,
            we need to convert value to reference using `&`

            or we can use `*` to dereference the `self.maximum_stack.last().unwrap()`
            and then can directly compare it to `value` be like
        */
        // if !self.maximum_stack.is_empty() && self.maximum_stack.last().unwrap() > &value {
        if !self.maximum_stack.is_empty() && *self.maximum_stack.last().unwrap() > value {
            self.maximum_stack.push(*self.maximum_stack.last().unwrap());
        } else {
            self.maximum_stack.push(value);
        }
    }

    fn pop(&mut self) {
        self.main_stack.pop();
        self.maximum_stack.pop();
    }

    fn max_value(&self) -> i32 {
        *self.maximum_stack.last().unwrap()
    }
}

fn main() {
    let mut stack = MaxStack::new();
    // Push 55 80 120 99 22 140 145
    stack.push(55);
    stack.push(80);
    stack.push(120);
    stack.push(99);
    stack.push(22);
    stack.push(140);
    stack.push(145);

    println!("Maximum value of stock : {:?}", stack.max_value());
    println!("After going one week back");
    stack.pop();
    println!("Maximum value of stock : {:?}", stack.max_value());
}
