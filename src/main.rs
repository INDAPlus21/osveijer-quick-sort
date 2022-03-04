fn main() {
    let mut line = String::with_capacity(500_000);
    std::io::stdin().lock().read_to_string(&mut line);

    let mut values: Vec<isize> = line
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();

    let length = values.len() as isize;

    sort(&mut values, 0, length as usize);

    for i in values {
        print!("{} ", i);
    }
    println!();
}

fn sort(nums: &mut Vec<isize>, start:usize, end:usize) {
    if start == end || end == 0 {return;}
    let pivot = nums[end-1];
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
