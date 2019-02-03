cosnt SETINEL:usize = 200000000;
fn main(){

}
fn merge(v:&mut Vec<usize>,count: &mut usize,n:usize,left:usize,mid:usize,right:usize) {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut l1 = vec![];
    let mut l2 = vec![];
    for i in 0..n1 { l1.push(v[left + i]);}
    for i in 0..n2 { l2.push(v[mid + i]);}
    l1[n1] =　SETINEL;
    l2[n2] =  SETINEL;
    let mut (i,j) = (0,0);
    for k in left..right {
        if l1[i] <= l2[j] { 
            i += 1;
            v[k] = l1[i];
        } else {
            j += 1;
            v[k] = l2[j];
        }
    }   
}

fn mergeSort(v:&mut Vec<usize>,count: &mut usize,n:usize,left:usize,right:usize) {
    if left + 1 < right {
        let mid = (left + right ) / 2;
        mergeSort(v, count, n, left, mid);
        mergeSort(v, count, n ,mid, right);
        merge(v,count,n,left,mid,right);
    }
}