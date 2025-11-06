use rand::rng;
use rand::seq::SliceRandom;

pub fn create_fruit_salad(
    add_fruits: &mut Vec<String>,
    num_fruits: usize,
    picked_fruits: Vec<String>,
) -> Vec<String> {
    let mut fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
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
