#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unreachable_code)]

fn partition(array: &mut [i32], low: usize, high: usize) -> usize {
    let mut i: usize = low;
    let mut j: usize = high;
    let pivot = array[low];

    loop {
        while i <= high && array[i] <= pivot {
            i += 1;
        }
        while array[j] > pivot {
            if j == 0 { break; }
            j -= 1;
        }

        if i < j {
            array.swap(i, j);
        } else {
            array.swap(low, j);
            return j;
        }
    }
}

fn quick_sort(array: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        print!("Array: ");
        for elem in array.iter() {
            print!("{elem} ");
        }
        println!();

        let j = partition(array, low, high);
        if j > 0 {
            quick_sort(array, low, j - 1);
        }
        quick_sort(array, j + 1, high);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![55, 3, 76, 19, 21, 23, 14, 1];
    let length: usize = v.len() - 1;
//    let mut v2: &mut Vec<i32> = &mut vec![]
    quick_sort(&mut v, 0, length);
}
