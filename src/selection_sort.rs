pub fn selection_sort(mut v: Vec<i64>, n: i64) -> Vec<i64> {
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if (v[j as usize] < v[minj as usize]) {
                minj = j;
            }
        }
        let sub = v[i as usize];
        v[i as usize] = v[minj as usize];
        v[minj as usize] = sub;
    }
    v
}
