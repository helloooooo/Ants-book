pub fn bubble_sort(mut v: Vec<i64>, n: i64) -> Vec<i64> {
    // let n = n - 1;
    let mut flag = true;
    while flag {
        flag = false;
        for j in 1..n {
            println!("n-j:{}", n - j);
            println!("n-1-j:{}", n - j);
            if v[(n - j) as usize] < v[(n - 1 - j) as usize] {
                let mut x = v[(n - j) as usize];
                v[(n - j) as usize] = v[(n - 1 - j) as usize];
                v[(n - 1 - j) as usize] = x;
                flag = true;
            }
        }
    }
    v
}
