fn get_item(index: usize) {
    // let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn get_sum(vec: Vec<i32>) -> i32 {
    let sum = vec.iter().sum();
    // if sum > 0 {
    //     Some(sum)
    // } else {
    //     None
    // }
    sum
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // let vec: Vec<i32> = vec![];

    // get_item(3);

    // Retrieve a value at a specific index
    // let third_value = vec[2];
    // println!("The third value in the vector is {}", third_value);

    // Retrieve the last value
    // let last_value = vec.last().unwrap();
    // println!("The last value in the vector is {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is {}", first_value),
        None => println!("The vector is empty!"),
    }

    let sum_value = get_sum(vec);
    println!("The sum value is {}", sum_value);
}
