struct Stack {
    stack_point: i32,
    stack: Vec<i32>,
}

impl Stack{
    fn add(&mut self){
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a+b);
        self.stack_point -= 1;
    }

    fn sub(&mut self){
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a - b);
        self.stack_point -= 1;
    }

    fn mul(&mut self){
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a * b);
        self.stack_point -= 1;
    }

    fn push(&mut self, value: i32){
        self.stack.push(value);
        self.stack_point += 1;
    }
}

fn main() {
    let contents: Vec<String> = input::read_numline();
    let mut stack:Stack = Stack{stack_point:0, stack: Vec::new()};
    for index in  0..contents.len() {
        let content = &contents[index];
        if content == "+" {
            stack.add();
        } else if content == "-" {
            stack.sub();
        } else if content == "*" {
            stack.mul();
        } else {
            let value = content.parse::<i32>().ok().unwrap();
            stack.push(value);
        }
    }

    println!("result is {}", stack.stack[0]);
}
