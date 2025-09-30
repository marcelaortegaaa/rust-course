fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn sum_elements(vec: &Vec<u8>) {
    let mut sum = 0;

    for e in vec {
        sum += e;
    }

    println!("The sum of the numbers in your vector is {}", sum);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // let vec: Vec<i32> = vec![];
    get_item(3);
    sum_elements(&vec);

    // Retrieve a value at a specific index
    // This panics
    // let third_value = vec[2];
    // println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    // This panics
    // let last_value = vec.last().unwrap();

    // This also panics, but with a message
    // let last_value: &_ = vec.last().expect("This vector is empty");
    // println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    // match vec.first() {
    //     Some(first_value) => println!("The first value in the vector is: {}", first_value),
    //     None => println!("The vector is empty!"),
    // }

    // Turned the match into an if-let just for practice
    // if let Some(first_value) = vec.first() {
    //     println!("The first value in the vector is: {}", first_value)
    // }
}
