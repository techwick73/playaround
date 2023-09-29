#[cfg(test)]
mod test {

    macro_rules! mad_skills {
        // Expressions
        // ($x: expr) => {
        //     format!("You sent an expression: {}", $x)
        // };

        // Types
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent a NON i32 type".to_string(),
            }
        };
    }

    #[test]
    fn test_declarative_macro() {
        // Expressions
        // let some_var = mad_skills!(1 + 2);
        // dbg!(some_var);

        // Types
        let some_var = mad_skills!(i32);
        dbg!(some_var);

        let some_var = mad_skills!(f32);
        dbg!(some_var);
    }
}
