// --------------------------------------------
//          Benchmark Testing
// --------------------------------------------

/*
    - Define 2 free functions

    "Free Function" is the function not tie to any struct
*/
pub fn sorting_algo_1<T: PartialOrd>(arr: &mut Vec<T>) {
    let mut swapped = false;
    for i in 0..(arr.len() - 1) {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i+1);
            swapped = true;
        }
    }
    if swapped {
        sorting_algo_1(arr);
    }
}

pub fn sorting_algo_2<T: Ord>(arr: &mut Vec<T>) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len{
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left)
    }
}