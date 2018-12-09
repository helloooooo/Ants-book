const MAX_SIZE:usize =  50;
struct Que {
    q:[i64;MAX_SIZE],
    head:usize,
    end:usize,
}
impl Que {
    fn new(n:usize) -> Self {
        Que {
            q:[0;MAX_SIZE],
            head:0,
            end:0,
        }
    }
    fn isEmpty(self) ->bool {
        self.head == self.end
    }
    fn isFull(self) -> bool {
        self.head == (end + 1) % MAX_SIZE
    }
    fn enqueue(&mut self,x:i64) -> () {
        if self.isFull() { panic!() }
        self.q[self.end] = x;
        if self.end + 1 == MAX_SIZE { self.end = 0;} else { self.end += 1; }
    }
    fn dequeue(&mut self) -> i64 {
        if self.isEmpty() { panic!() }
        let x = self.q[head];
        if self.head + 1 == MAX_SIZE { self.head = 0; } else { self.head += 1;}
        x
    }
}
pub fn que(v:Vec<i64>,n:i64,q:i64) -> {

}