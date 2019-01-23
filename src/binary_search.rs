fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
fn main(){
    let n = read::<i64>();
    let mut sn = read_vec::<i64>();
    let t = read::<i64>();
    let tn = read_vec::<i64>();

    binary_search(n,sn, t, tn);
}

pub fn binary_search(n:i64,mut sn:Vec<i64>,t:i64,tn:Vec<i64>)->i64{
    sn.sort();
    let mut ans = 0;
    for b in tn {
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = (left + right) / 2;
            if sn[mid as usize] == b {
                ans += 1;
                break;
            } else if b < sn[mid as usize] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }        
    }
    return ans;
}