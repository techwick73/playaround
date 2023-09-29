#[cfg(test)]
mod test {

    /* Letting the compiler highlight mistakes as opposed to running code with tests
    Therefor using:
    #[allow(dead_code, unused_variables, clippy::needless_late_init)]
    ahead of each function declaration
    */
    #[allow(dead_code, unused_variables, clippy::needless_late_init)]
    fn example_0() {
        let r: &i32;

        // with the scope restricted i.e. uncomment the brackets we get 'x' does not live long enough - borrowed value does not live long enough
        //   {
        let x = 5;
        r = &x;
        // }
        println!("r: {}", r);
    }

    #[allow(dead_code, unused_variables, clippy::needless_late_init)]
    fn example_1() {
        // Allocate space in memory
        let highest_age: i32;

        // Init vars
        let alice_age = 20;
        let bob_age = 21;

        // Call function
        highest_age = largest(&alice_age, &bob_age);

        // Print output
        println!("The highest age is {}", highest_age);

        // Passing in the values as references is best practice as it saves having to pass entire values which could theoretically be quite large
        // We're just passing pointers
        // However we're in the situation of having to pass back a value
        // What to do ?
        // We de-referene the required reference and pass back the value ... see example_2 for a cleaner way of doing this
        fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
            if compare_1 > compare_2 {
                *compare_1
            } else {
                *compare_2
            }
        }
    }

    // Here we start with the mandated requirement of 'largest' having to returning a reference not a value
    #[allow(dead_code, unused_variables, clippy::needless_late_init)]
    fn example_2() {
        // Allocate space in memory - but this time as a reference
        let highest_age: &i32;

        // Init vars
        let alice_age = 20;
        let bob_age = 21;

        // Call function
        highest_age = largest(&alice_age, &bob_age);

        // Print output
        println!("The highest age is {}", highest_age);

        // Now we introduce the concept of lifetimes
        // We tage the function name with <'a>, the use &'a for all the references in the function signature
        // Below we have 2 lifetimes: 'a and 'b. We state they're the same by using 'b : 'a
        // The lifetime of the return element is always that of the shortest of the passed in elements
        fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> &'a i32 {
            if compare_1 > compare_2 {
                compare_1
            } else {
                compare_2
            }
        }
    }

    // Lifetimes with generics and structs
    #[allow(dead_code, unused_variables, clippy::needless_late_init)]
    fn example_3_generics() {
        // Allocate space in memory - but this time as a reference
        let highest_age: &i32;

        // Init vars
        let alice_age = 20;
        let bob_age = 21;

        // Call function
        highest_age = largest(&alice_age, &bob_age);

        // Print output
        println!("The highest age is {}", highest_age);

        // Change the types of the passed in and returned elements to be generic
        // This immediately generates a "binary operation > cannot be applied to type &T"
        // so we need to state that the type implements the PartialOrd trait
        fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
            if compare_1 > compare_2 {
                compare_1
            } else {
                compare_2
            }
        }
    }

    // Structs
    // Defining a struct with references causes the compiler to demand lifetime parameters
    #[allow(dead_code)]
    struct Person<'p> {
        name: &'p str,
        points: &'p f32,
    }

    // Lifetimes with generics and structs
    #[allow(dead_code, unused_variables, clippy::needless_late_init)]
    fn example_4_with_structs() {
        // Allocate space in memory - but this time as a reference
        let highest_val: &f32;

        // Init vars
        let alice: Person = Person {
            name: "alice",
            points: &50.2,
        };
        let bob: Person = Person {
            name: "bob",
            points: &40.5,
        };

        // Call function
        highest_val = largest::<f32>(alice.points, bob.points);

        // Print output
        println!("The highest age is {}", highest_val);

        // Change the types of the passed in and returned elements to be generic
        // This immediately generates a "binary operation > cannot be applied to type &T"
        // so we need to state that the type implements the PartialOrd trait
        fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
            if compare_1 > compare_2 {
                compare_1
            } else {
                compare_2
            }
        }
    }
}
