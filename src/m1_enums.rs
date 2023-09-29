#[cfg(test)]
mod test {

    #[derive(Debug)]
    #[allow(dead_code)]
    enum CarColour {
        Red,
        Green,
        Blue,
        Silver,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum GivenResult<T, E> {
        OK(T),
        Err(E),
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum GivenOption<T> {
        None,
        Some(T),
    }

    #[allow(dead_code)]
    fn create_car_colour_blue() -> CarColour {
        let my_car_colour: CarColour = CarColour::Blue;
        my_car_colour
    }

    #[allow(dead_code)]
    fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
        if num_check < 5 {
            GivenResult::OK(num_check)
        } else {
            GivenResult::Err("Number is greater than 5".to_string())
        }
    }

    #[allow(dead_code)]
    fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
        if num_check < 5 {
            Ok(num_check)
        } else {
            Err("Number is greater than 5".to_string())
        }
    }

    #[allow(dead_code)]
    fn remainder_zero(num_check: f32) -> GivenOption<f32> {
        let remainder: f32 = num_check % 10.0;
        if remainder == 0.0 {
            GivenOption::None
        } else {
            GivenOption::Some(remainder)
        }
    }

    #[allow(dead_code)]
    fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
        let remainder: f32 = num_check % 10.0;
        if remainder == 0.0 {
            None
        } else {
            Some(remainder)
        }
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_enums() {
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_five_res);

        let under_five_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(under_five_res);

        let remainder_zero_res: GivenOption<f32> = remainder_zero(12.2);
        dbg!(remainder_zero_res);

        let remainder_zero_res: GivenOption<f32> = remainder_zero(10.0);
        dbg!(remainder_zero_res);

        let under_five_res: Result<u8, String> = check_under_five_built_in(2);
        dbg!(under_five_res);

        let under_five_res: Result<u8, String> = check_under_five_built_in(7);
        dbg!(under_five_res);

        let remainder_zero_res: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(remainder_zero_res);

        let remainder_zero_res: Option<f32> = remainder_zero_built_in(10.0);
        dbg!(remainder_zero_res);
    }
}
