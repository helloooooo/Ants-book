fn partition(v:&mut Vec<i64>){
    let x = v[v.len()-1];
    println!("{}",x);
    let p = 0;
    let mut i:i64 = p - 1; 
    let mut temp_v = 0;
    let r:i64 = (v.len()-1) as i64;
    for j in p..r {
        if v[j as usize] <= x {
            i += 1;
            temp_v = v[i as usize];
            println!("{}",temp_v); 
            v[i as usize] = v[j as usize]; 
            v[j as  usize] = temp_v;
        }
    }
    temp_v = v[(i+1) as usize]; v[(i+1) as usize] = v[r as usize]; v[r as usize] = temp_v;
}

#[test]

fn partition_test(){
    let mut v = vec![3,9,8,1,5,6,2,5];
    partition(&mut v);
    assert_eq!(vec![3,1,5,2,5,6,9,8],v);
}