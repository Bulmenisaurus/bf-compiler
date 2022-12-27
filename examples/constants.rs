use std::collections::HashMap;

//
fn recursive_get_representation(num: i32, constants: &HashMap<i32, String>) -> String {
    let mut options: Vec<String> = vec![];

    // if already computed, just return it
    let computed_constant = constants.get(&num);
    match computed_constant {
        Some(string) => return string.to_string(),
        None => {}
    }

    // otherwise, for possible a + b
    // since a + b is equivalent to b + a we will assume a <= b
    let num_possibilities = num / 2;
    for a in 1..=num_possibilities {
        let b = num - a;

        let representation_a = recursive_get_representation(a, constants);
        let representation_b = recursive_get_representation(b, constants);

        options.push(representation_a + &representation_b);
    }
    let mut sorted_options = options;
    sorted_options.sort_by(|a, b| a.len().cmp(&b.len()));

    sorted_options[0].clone()
}

fn get_bf_representations() {
    let mut computed_numbers: HashMap<i32, String> = HashMap::new();

    computed_numbers.insert(255, String::from("-"));
    computed_numbers.insert(1, String::from("+"));
    computed_numbers.insert(0, String::from(""));

    for i in 0..=255 {
        let representation = recursive_get_representation(i, &computed_numbers);
        computed_numbers.insert(i, representation.clone());

        println!("[{}] {:?}", i, representation);
    }
}

fn main() {
    get_bf_representations();
}
