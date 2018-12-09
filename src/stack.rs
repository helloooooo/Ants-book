struct Stack {
    stack: Vec<i64>,
    n:i64,
}
impl Stack {
    fn new(n:i64) -> Self {
        Stack {
            stack: Vec::new(),
            n,
        }
    }
    fn push(&mut self,x:i64) -> (){
        self.stack.push(x);
    }
    fn pop(&mut self) -> i64{
        let v = self.stack.clone();
        let t = v.last().unwrap();
        self.stack.pop();
        println!("{:?}",self.stack);
        *t
    }
}
pub fn stack(input:Vec<char>,n:i64) -> i64 {
    let mut s = Stack::new(n);
    for v in input {
        if v == '+' {
            let a = s.pop();
            let b = s.pop();
            s.push(a + b);
        } else if v == '-' {
            let a = s.pop();
            let b = s.pop();
            s.push(b - a);
        } else if v == '*' {
            let a = s.pop();
            let b = s.pop();
            s.push(a * b);
        } else {
            s.push(v.to_digit(10).unwrap() as i64);
        }
    }
    *s.stack.last().unwrap()
}