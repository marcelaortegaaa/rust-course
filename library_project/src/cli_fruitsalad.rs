use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom};

pub fn create_fruit_salad(
    add_fruits: &mut Vec<String>,
    num_fruits: usize,
    picked_fruits: Vec<String>,
) -> Vec<String> {
    let mut fruits = vec![
        "arbutus".to_string(),
        "loquat".to_string(),
        "strawberry".to_string(),
        "pomegranate".to_string(),
        "fig".to_string(),
        "cherry".to_string(),
        "orange".to_string(),
        "pear".to_string(),
        "peach".to_string(),
        "apple".to_string(),
    ];

    fruits.append(add_fruits);

    let mut rng = rng();
    fruits.shuffle(&mut rng);

    let mut fruits: Vec<String> = fruits
        .into_iter()
        .filter(|s| picked_fruits.is_empty() || picked_fruits.contains(s))
        .take(num_fruits)
        .collect();

    fruits.sort();
    fruits
}

pub fn add_surprise(surprise: bool) -> String {
    let mut rng = rng();
    let surprise_list = [
        "syrup".to_string(),
        "ice scream".to_string(),
        "cheese".to_string(),
        "chocolate".to_string(),
    ];

    let s = if surprise {
        surprise_list.choose(&mut rng).unwrap()
    } else {
        ""
    };

    s.to_string()
}
