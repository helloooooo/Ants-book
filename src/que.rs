use std::cmp::min;
use std::collections::VecDeque;

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
        self.head == (self.end + 1) % MAX_SIZE
    }
    fn enqueue(&mut self,x:i64) -> () {
        if self.isFull() { panic!() }
        self.q[self.end] = x;
        if self.end + 1 == MAX_SIZE { self.end = 0;} else { self.end += 1; }
    }
    fn dequeue(&mut self) -> i64 {
        if self.isEmpty() { panic!() }
        let x = self.q[self.head];
        if self.head + 1 == MAX_SIZE { self.head = 0; } else { self.head += 1;}
        x
    }
}
pub fn que(v:Vec<(String,i64)>,n:i64,t:i64)-> Vec<(String,i64)>{
    let mut q:VecDeque<(String,i64)> = VecDeque::new();
    // let mut ans:VecDeque<(String,i64)> = VecDeque::new();
    let mut time = 0;
    for v_ in v {
        q.push_front((v_.0,v_.1));
    }
    let mut ans =vec![];
    println!("{:?}",q);
    while q.len() != 0 {
        let (ss,mut remain) = q.pop_front().unwrap();
        let current = min(remain,t);
        remain -= current;
        time += current;
        if remain > 0 {
            q.push_back((ss,remain));
        } else {
            ans.push((ss,current));
        }
    }
    ans
}