mod scaneer;
use scaneer::*;
fn main(){
    let n = read::<i64>();
    let s = read::<String>();
    let s :Vec<char> = s.chars().collect();
    let mut cs:Vec<char> = s.clone();
    cs.reverse();
    let mut a = 0;
    let mut b = n -1;
    while a <= b {
        let mut left = false;
        for i in 0..b-a {
            if s[(a + i ) as usize] < s[(b -i ) as usize] {
                left = true;
                break;
            }  else if s[(a + i ) as usize] > s[(b -i ) as usize] {
                left = false;
                break;
            }
        }
        if left {
            
            print!("{}",s[a as usize]);
            a = a + 1;
        } else {
           
            print!("{}",s[b as usize]);
             b = b - 1;
        }
    }
}
