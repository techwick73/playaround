#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn tests_hashmap() {
        let person_1 = "alice";
        let person_2 = "bob";

        let mut results_hm: HashMap<&str, u32> = HashMap::new();

        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 51);

        // Hashmaps always return an Option
        let test_res = results_hm.get(person_1);
        dbg!(test_res);

        if results_hm.contains_key("alice") {
            dbg!("Alice is here!");
        }
    }

    #[test]
    fn tests_hashset() {
        let mut names_hm: HashSet<&str> = HashSet::new();

        names_hm.insert("alice");
        names_hm.insert("bob");
        names_hm.insert("victor");

        if names_hm.contains("bob") {
            dbg!("Bob is here!");
        }
    }
}
