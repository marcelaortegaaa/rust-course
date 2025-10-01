// enum FileSize {
//     Bytes(u64),
//     Kilobytes(u64),
//     Megabytes(u64),
//     Gigabytes(u64),
//     Terabytes(u64),
// }

// fn format_size(size: u64) -> String {
//     let filesize = match size {
//         0..=999 => FileSize::Bytes(size),
//         1000..=999_999 => FileSize::Kilobytes(size),
//         1_000_000..=999_999_999 => FileSize::Megabytes(size),
//         1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size),
//         _ => FileSize::Terabytes(size),
//     };

//     match filesize {
//         FileSize::Bytes(bytes) => format!("{} bytes", bytes),
//         FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1_000.0),
//         FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1_000_000.0),
//         FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1_000_000_000.0),
//         FileSize::Terabytes(gb) => format!("{:.2} TB", gb as f64 / 1_000_000_000_000.0),
//     }
// }

enum Shape {
    Circle(f64),
    Square(f64),
    Triangle((f64, f64)),
}

fn biggest_area(shapes_vector: &Vec<Shape>) -> (&Shape, f64) {
    let biggest_shape = shapes_vector
        .iter()
        .map(|shape| {
            let area = match shape {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(length) => length * length,
                Shape::Triangle((base, height)) => base * height / 2.0,
            };
            (shape, area)
        })
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();

    return biggest_shape;
}

fn main() {
    // let result = format_size(2594);
    // println!("{}", result)

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle((2.0, 5.0)),
    ];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle((base, height)) => base * height / 2.0,
        })
        .sum();

    // I wrote the code for the area twice, maybe fill fix later

    let biggest_area = biggest_area(&shapes);
    let biggest_area_shape_name = match biggest_area.0 {
        Shape::Circle(_) => "Circle",
        Shape::Square(_) => "Square",
        Shape::Triangle(_) => "Triangle",
    };
    // This doesn't feel right. Check later

    println!("Total area: {:.2} square units", total_area);
    println!(
        "Biggest area: A {} of {:.2} square units",
        biggest_area_shape_name, biggest_area.1
    );
}
