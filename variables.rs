fn main() {
    let mut array: [i32; 5] = [5, 4, 3, 2, 1];
    let array_iterator = array.iter();
    for item in array_iterator {
        println!("item {}", item);
    }
    for i in 0..5 {
        array[i] = i as i32;
        println!("item {}", array[i]);
    }

    println!("Rust slicing below");
    println!("Slice = {:?}", &array[1..3]);
}
