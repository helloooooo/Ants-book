extern crate Ants_book;


#[cfg(test)]
mod tests{
    use Ants_book::insertionsort::*;
    use Ants_book::bubblesort::*;
    #[test]
    fn ins1(){
        // let mut v = read_vec::<i64>();
        let v = vec![8,3,1,5,2,1];
        let mut sub = v.clone();
        let l = v.clone().len();
        let ans = insertion_sort(v,l as i64);
        sub.sort();
        assert_eq!(ans,sub);
    }

    #[test]
    fn bubble(){
        let mut v  = vec![5,3,2,4,1];
        v.sort();
        assert_eq!(bubble_sort(vec![5,3,2,4,1],5), v);
    }
    #[test]
    fn select(){
        use Ants_book::selection_sort::*;
        let mut v = vec![5,6,4,2,1,3];
        v.sort();
        assert_eq!(vec![1,2,3,4,5,6],v);
    }
    #[test]
    fn stack_test(){
        use Ants_book::stack::*;
        let v = vec!['1','2','+','3','4','-','*'];
        assert_eq!(stack(v,6),-3);
    }

}
