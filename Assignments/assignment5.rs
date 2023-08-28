use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K, V> {
    fn sort_by_key(&mut self);
}

// Implement SortByKey for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&mut self) {
        let mut sorted_pairs: Vec<(K, V)> = self.drain().collect();
        sorted_pairs.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));
        
        for (key, value) in sorted_pairs {
            self.insert(key, value);
        }
    }
}

fn main() {
    // Create a new HashMap instance
    let mut my_map: HashMap<i32, &str> = HashMap::new();
    
    // Add key-value pairs to the HashMap
    my_map.insert(3, "three");
    my_map.insert(1, "one");
    my_map.insert(2, "two");
    
    // Print the initial state of the map
    println!("Before sorting: {:?}", my_map);
    
    // Sort the map by key
    my_map.sort_by_key();
    
    // Print the sorted map
    println!("After sorting: {:?}", my_map);
}
