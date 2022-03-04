use std::io::Read;

fn main() {
    let mut line = String::with_capacity(500_000);
    std::io::stdin().lock().read_to_string(&mut line).expect("unable to read input");

    let mut values: Vec<isize> = line
        .split_whitespace()
        .skip(1)
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();

    let length = values.len();

    sort(&mut values, 0, length);

    for i in values {
        print!("{} ", i);
    }
    println!();
}

fn sort(nums: &mut Vec<isize>, start:usize, end:usize) {
    if start >= end || end == 0 {return;}
    // insertion sort for small slices
    if end - start < 15 {insertion_sort(nums, start, end);}
    let pivot = get_pivot(start, end-1, (start+end-1)/2, nums);
    let mut cur_index = start;
    for i in start..end {
        if nums[i] <= pivot {
            if i != cur_index {nums.swap(i, cur_index);}
            cur_index += 1;
        }
    }
    sort(nums, start, cur_index - 1);
    sort(nums, cur_index, end);
}

/// Get pivot as median of first last andd middle value. Also make sure pivot is in last
fn get_pivot(first: usize, last: usize, middle: usize, nums: &mut Vec<isize>) -> isize {
    let (a,b,c) = (nums[first],nums[last],nums[middle]);
    if a > b {
        if c > a {
            nums.swap(first,last);
            return a;
        }
        else if b > c {return b;}
        else {
            nums.swap(middle,last);
            return c;
        }
    }
    else {
        if c < a {
            nums.swap(first,last);
            return a;
        }
        else if b < c {return b;}
        else {
            nums.swap(middle,last);
            return c;
        }
    }
}


fn insertion_sort(nums: &mut Vec<isize>, start:usize, end:usize) {
    for i in start..end {
        let pivot = nums[i];
        for j in (i..start).rev() {
            if pivot < nums[j - 1] {
                nums[j] = nums[j - 1];
                if j - 1 == start {
                    nums[j - 1] = pivot;
                    break;
                }
            }
            else {
                nums[j] = pivot;
                break;
            }
        }
    }
}