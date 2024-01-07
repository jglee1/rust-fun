fn add_value_to_vector_at_beginning_at_end(v: &mut Vec<i32>, value: i32) -> Vec<i32> {
    v.insert(0, value);
    v.push(value);
    v.to_vec()
}

fn combine_two_vectors(v1: &mut Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    v1.extend(v2);
    v1.to_vec()
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("v: {:?}", v);

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("v: {:?}", v);

    // append adds the given vector to the vector, requires the given vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("v: {:?}", v);

    // insert items at a given index
    v.insert(0, 100);
    println!("v: {:?}", v);

    let new_v = add_value_to_vector_at_beginning_at_end(&mut v, 200);
    println!("new_v: {:?}", new_v);

    let v2 = vec![300, 400];
    println!("v2: {:?}", v2);
    let combined_v = combine_two_vectors(&mut v, v2);
    println!("combined_v: {:?}", combined_v);
}
