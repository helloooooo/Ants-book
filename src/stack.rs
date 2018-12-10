const MAX_SIZE: usize = 50;
struct Stack {
    stack: [i64; MAX_SIZE],
    top: usize,
    n: usize,
}
impl Stack {
    fn new(n: usize) -> Self {
        Stack {
            stack: [0; MAX_SIZE],
            top: 0,
            n,
        }
    }
    fn isEmpty(&self) -> bool {
        self.top == 0
    }
    fn isFull(&self) -> bool {
        self.top == self.n - 1
    }
    fn push(&mut self, x: i64) -> () {
        if self.isFull() {
            panic!()
        }
        self.top += 1;
        self.stack[self.top] = x;
    }
    fn pop(&mut self) -> i64 {
        if self.isEmpty() {
            panic!()
        }
        self.top -= 1;
        return self.stack[self.top + 1];
    }
}
pub fn stack(input: Vec<char>, n: i64) -> i64 {
    let mut s = Stack::new(n as usize);
    for v in input {
        if v == '+' {
            let a = s.pop();
            let b = s.pop();
            println!("add-----------");
            println!("a:{}", a);
            println!("b:{}", b);
            println!("addend-----------");
            s.push(a + b);
        } else if v == '-' {
            let a = s.pop();
            let b = s.pop();
            println!("diff-----------");
            println!("a:{}", a);
            println!("b:{}", b);
            println!("diffend-----------");
            s.push(b - a);
        } else if v == '*' {
            let a = s.pop();
            let b = s.pop();
            println!("****-----------");
            println!("a:{}", a);
            println!("b:{}", b);
            println!("****end-----------");
            s.push(a * b);
        } else {
            s.push(v.to_digit(10).unwrap() as i64);
        }
    }
    s.stack[s.top]
}
