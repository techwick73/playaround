#[cfg(test)]
mod test {

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        Quit,
        ChangeColour(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    #[allow(dead_code)]
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => {
                println!("I quit")
            }
            Message::ChangeColour(red, blue, green) => {
                println!("red {} blue {} green {}", red, blue, green)
            }
            Message::Move { x, y } => {
                println!("x {} y {} ", x, y)
            }
            Message::Write(a_string) => {
                println!("{}", a_string)
            }
        }
    }

    #[test]
    fn test_match_message() {
        process_message(Message::Quit);
        process_message(Message::ChangeColour(10, 20, 255));
        process_message(Message::Move { x: 12, y: 13 });
        process_message(Message::Write("hello there".to_string()));
    }

    #[test]
    fn test_match_literal() {
        let number = 20;

        let res = match number {
            1 => "This is the first",
            2 | 3 | 5 | 20 => "We found it in the sequence",
            _ => "All other cases",
        };

        println!("{}", res);
    }
    #[test]
    fn test_match_option() {
        let some_num: Option<i32> = Some(10);

        // if let some will unpack the some value of an option
        if let Some(i) = some_num {
            println!("{}", i)
        }
        // let prob_none: Option<i32> = None;

        // let res = match some_num {
        //     Some(i) => i,
        //     None => panic!("There was a problem"),
        // };
        // println!("{:?}", some_num);
        // println!("{}", res);
    }
    #[allow(unused_variables)]
    #[test]
    fn test_match_result() {
        let some_result: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("There was an error");

        let res = match some_result {
            Ok(i) => i,
            Err(e) => panic!("{}", e),
        };

        println!("{}", res);

        let my_int = if let Ok(i) = some_result {
            i
        } else {
            panic!("there was a problem")
        };
        println!("{}", my_int)
    }

    #[test]
    #[allow(unused_variables)]

    fn test_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let location = Location { x: 0, y: 20 };

        match location {
            Location { x, y: 0 } => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("Neither X or Y is on the axis"),
        };
    }
}
