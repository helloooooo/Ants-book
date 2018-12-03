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
    let mut v = read_vec::<i64>();
    let l = v.clone().len();
    let ans = insertion_sort(v,l);
    println!("---------");
    println!("{:?}",ans);
}
fn insertion_sort(mut a:Vec<i64>,n:usize) -> Vec<i64>{
    for i in 1..(n as usize) {
        let v = a[i];
        let mut j = i - 1;
        while j >= 0 &&  a[j] > v {
            a[j + 1] = a[j];
            j -= 1;
        }
        a[j + 1] = v;
        println!("{:?}",a);
    }
    a
}

#[test]
fn ins1(){
    // let mut v = read_vec::<i64>();
    let mut v = vec![8,3,1,5,2,1];
    let ans = insertionSort(v,v.len());
    v.sort();
    assert_eq!(ans,v);
}