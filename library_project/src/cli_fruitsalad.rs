use rand::rng;
use rand::seq::SliceRandom;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
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

    let mut rng = rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}
