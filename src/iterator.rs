use scaneer::*;

fn main(){
    let v = read_vec::<char>();
    iterators(v);
}

pub fn iterators(v:Vec<char>) -> (String,String) {
    let s = v.iter().collect();
    let a = v.iter().map(|&x| x.to_digit(10).unwrap() + 1)
        .map(|x| std::char::from_digit(x, 10).unwrap())
        .collect();
    (s,a)
}
