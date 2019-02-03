use scaneer::*;

fn main(){
    let n = read::<usize>();
    let an = read_vec::<usize>();
    let m = read::<usize>();
    let bn = read::<usize>();
}

fn all_sum(v:&Vec<usize>,i:usize,m:usize) -> bool {
    if m == 0 { return true;}
    if i >= v.len()-1 { return false;}
    let res = all_sum(v,i+1,m) || all_sum(v, i+1, m-v[i]);
    return res;
}

#[test]
fn all_sum_test(){
    assert_eq!(all_sum(&vec![1,5,7,10,21], 0, 17),true);
}