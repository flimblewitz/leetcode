fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(3);
    min_stack.pop();
    println!("{}", min_stack.top());
    println!("{}", min_stack.get_min());
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
struct MinStack {
    vals: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            vals: vec![],
            mins: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.vals.push(val);
        self.mins
            .push(val.min(*self.mins.last().unwrap_or(&std::i32::MAX)))
    }

    fn pop(&mut self) {
        self.vals.pop();
        self.mins.pop();
    }

    fn top(&self) -> i32 {
        *self.vals.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}
