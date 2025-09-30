fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn sum_elements(vec: &[i32]) -> i32 {
    // let mut sum = 0;

    // for e in vec {
    //     sum += e;
    // }

    vec.iter().sum()
}

fn add_first_last<T: Clone>(vec: &mut Vec<T>, item: T) {
    vec.insert(0, item.clone());
    vec.push(item);
}

fn append_vector<T: Clone>(main_vector: &mut Vec<T>, appended_vector: &mut Vec<T>) {
    main_vector.append(appended_vector);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    // let vec: Vec<i32> = vec![];

    get_item(3);
    let sum = sum_elements(&vec);
    println!("The sum of the numbers in your vector is {}", sum);

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

    // ----------

    let mut v = vec![1, 2, 3];
    let mut hutter = vec![11, 12, 13]; // getting it's numbers sucked like it's nosferatu

    v.push(4);
    println!("{:?}", v);

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]}

    add_first_last(&mut v, 9);
    println!("{:?}", v);

    append_vector(&mut v, &mut hutter);
    println!("{:?} and {:?}", v, hutter);
}
