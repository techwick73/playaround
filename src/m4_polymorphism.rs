#[cfg(test)]
mod test {
    use ethers::types::Address;
    use std::str::FromStr;

    // Create a trait on types Address and str
    // Each now can invoke the function convert_address
    trait EthereumAddress {
        fn convert_address(&self) -> Result<Address, &'static str>;
    }

    // Implement convert_address for type &str
    // This will return either an Address or a str with an error message
    impl EthereumAddress for &str {
        fn convert_address(&self) -> Result<Address, &'static str> {
            match Address::from_str(self) {
                Ok(address) => Ok(address),
                Err(_) => Err("Invalid Ethereum address"),
            }
        }
    }

    // Implement convert_address for type Address
    // This will always return a string - itself
    impl EthereumAddress for Address {
        fn convert_address(&self) -> Result<Address, &'static str> {
            Ok(*self)
        }
    }

    #[allow(dead_code)]
    fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
        let converted_address: Address = address.convert_address().unwrap();
        converted_address
    }

    #[test]
    fn test_poly() {
        // Pass in a string copied in from etherscan and convert it into a type Address
        let addr = Address::from_str("0xB62E45c3Df611dcE236A6Ddc7A493d79F9DFadEf").unwrap();

        // Assert that addr (type Address) is equal to itself
        // We don't really need to do this but just proving that Address::from_str is actually converting to Address from a string
        assert_eq!(
            addr,
            Address::from_str("0xB62E45c3Df611dcE236A6Ddc7A493d79F9DFadEf").unwrap()
        );

        // Test1: Calling get_ethereum_data by passing in a type of Address
        // Test 2: Calling get_ethereum_data by passing in a type of string
        // This demonstrates polymorphism

        // Test 1
        // Pass in address as type address
        let new_addr = get_ethereum_data(addr);
        assert_eq!(
            new_addr,
            Address::from_str("0xB62E45c3Df611dcE236A6Ddc7A493d79F9DFadEf").unwrap()
        );

        // Test 2
        // Pass in address as type string
        let new_addr = get_ethereum_data("0xB62E45c3Df611dcE236A6Ddc7A493d79F9DFadEf");
        assert_eq!(
            new_addr,
            Address::from_str("0xB62E45c3Df611dcE236A6Ddc7A493d79F9DFadEf").unwrap()
        );
    }
}
