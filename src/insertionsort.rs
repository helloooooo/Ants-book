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
fn main() {
    let v = read_vec::<i64>();
    let l = v.clone().len();
    let ans = insertion_sort(v, l as i64);
    println!("---------");
    println!("{:?}", ans);
}
pub fn insertion_sort(mut a: Vec<i64>, n: i64) -> Vec<i64> {
    for i in 1..n {
        let v = a[i as usize];
        let mut j: i64 = i - 1;
        while j >= 0 && a[j as usize] > v {
            println!("j:{}", j);
            a[j as usize + 1] = a[j as usize];
            println!("1");
            j -= 1;
        }
        a[(j + 1) as usize] = v;
    }
    a
}
#[cfg(test)]
#[test]
fn ins1() {
    // let mut v = read_vec::<i64>();
    let v = vec![8, 3, 1, 5, 2, 1];
    let mut sub = v.clone();
    let l = v.clone().len();
    let ans = insertion_sort(v, l as i64);
    sub.sort();
    assert_eq!(ans, sub);
}
