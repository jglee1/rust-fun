fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice = &numbers[..]; // creates a slice of all elements in numbers
    println!("slice = {:?}", slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let slice = &mut numbers[..]; // creates a mutable slice of all elements in numbers
    slice[0] = 10;
    // This would fail without a new declaration of numbers
    // let mut numbers = vec![1, 2, 3];
    // let other_slice = &mut numbers[..];
    println!("slice = {:?}", slice);
}

fn main() {
    // slices and vectors are similar. But slices are immutable depending on how they are borrowed
    ownership();
    modifiable();
}
