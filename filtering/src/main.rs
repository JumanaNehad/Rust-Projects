#![deny(clippy::all)]

struct FilterCondition {
    condition: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.condition
    }
}

fn custom_filter(filter: FilterCondition, collection: Vec<i32>) -> Vec<i32> {
    collection
        .into_iter()
        .filter(|x| filter.is_match(x))
        .collect()

    //or
    //     let mut result = Vec::new();
    // for item in collection {
    //     if filter.is_match(&item) {
    //         result.push(item);
    //     }
    // }
    // result
}

fn main() {
    let v = vec![1, 2, 4, 5, 6];

    let obj = FilterCondition { condition: 2 };

    //Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
    let result = custom_filter(obj, v);
    println!("Filtered Collection: {:?}", result);
}
