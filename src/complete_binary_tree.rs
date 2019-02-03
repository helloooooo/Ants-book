fn main(){

}
fn complete_binary_tree(v:&Vec<i64>,H:i64) {
    for x in v {
        if parent(x) >= 1 { println!("{}",v[parent(x)]);}
        if left(x) <= H { println!("{}",v[left(x)]); }
        if right(x) <= H { println!("{}",v[right(x)]);}
    }
}
fn parent(i:i64) -> i64 {
    return i / 2
}
fn left(i:i64) -> i64 {
    return 2 * i
}
fn right(i:i64) -> i64 {
    return 2 * i + 1
}