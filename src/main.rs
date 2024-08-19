fn main() {
    println!("Hello, world!");
}

use alloy_primitives::B256;
use reqwest::Body;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_foo_fails() {
        // want: 0x79adbd5094e60c1bc2b963678ff44695d1430b8ccff0b1cd57c03a7f63567822
        // got : 0xae33576402a61ff3a4a241b4a91d20ce98b70aa7bf0f388fb857111ec83aef73
        let a = alloy_primitives::keccak256("testFoo()");
        let b = alloy_primitives::keccak256("test_Foo()");
        println!("testFoo() : {:?}", a);
        println!("test_Foo(): {:?}", b);
        assert_eq!(format!("{a:?}"), String::from("0x79adbd5094e60c1bc2b963678ff44695d1430b8ccff0b1cd57c03a7f63567822"));
        assert_eq!(format!("{b:?}"), String::from("0x45c48c2bd4afc6adc7884fe296b9af10e234ddbc44f2f99f40cfb8b6391e9798"));
    }
}
