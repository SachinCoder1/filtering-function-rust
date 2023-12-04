struct FilterCondition<T> {
    condition_value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition_value == item
    }
}

fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
{
    collection.into_iter().filter(|item| condition.is_match(item)).collect()
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5];
    let condition = FilterCondition { condition_value: 3 };

    let filtered_collection = custom_filter(collection, &condition);
    println!("Filtered result: {:?}", filtered_collection);
}


fn main() {
    println!("Hello, world!");
}
