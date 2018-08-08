mod scaneer;
use scaneer::*;


const INF: i64 = 100000;
struct Points {
    start:(i64,i64),
    goal :(i64,i64),
    n : i64,
    m:i64,
}
impl Points {
    fn bfs(self, d:Vec<Vec<i32>>,field:Vec<Vec<char>>) -> i64 {
        let mut v  = Vec::new();
        let mut init_d:Vec<Vec<i64>> = d.iter().map(|x| x.iter().map(|y|INF).collect()).collect();
        v.push(self.start);
        init_d[self.start.0 as usize][self.start.1 as usize] = 0;
        let dx = vec![1,0,-1,0];
        let dy = vec![0,1,0,-1];
        while !v.is_empty(){
            let v_pop = v.pop().unwrap();
            if v_pop.0 == self.goal.0 && v_pop.1 == self.goal.1 { break;}
            let moves:Vec<(&i64,&i64)> = dx.iter().zip(dy.iter()).collect();
            for j in 0..4{
                let nx = v_pop.0 + moves[j as usize].0;
                let ny = v_pop.1 + moves[j as usize].1;
                if 0 <= nx && nx < self.n && 0 <= ny && ny < self.m && field[nx as usize][ny as usize] != '#' && init_d[nx as usize][ny as usize] == INF {
                    v.push((nx,ny));
                    init_d[nx as usize][ny as usize] = init_d[v_pop.0 as usize][v_pop.1 as usize] + 1;
                }
            }
        }
        init_d[self.goal.0 as usize][self.goal.1 as usize]
    }
}

fn main(){
    let nm = read_vec::<i64>();
    let (n,m) = (nm[0],nm[1]);
    let mut field = Vec::new();
    for _ in 0..n {
        field.push(read::<String>().chars().collect());
    }
    let start = read_vec::<i64>();
    let goal = read_vec::<i64>();
    let d = vec![vec![0;n as usize];m as usize];
    let p  = Points {
        start: (start[0],start[1]),
        goal: (goal[0],goal[1]),
        n: n,
        m: m,
    };
    let ans = p.bfs(d,field);
    println!("{}",ans);
}