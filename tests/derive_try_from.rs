#[macro_use] extern crate conv;

use conv::TryFrom;

#[derive(Debug, PartialEq)]
enum Get { Up, Down, AllAround }

TryFrom! { (u8) enum Get { Up, Down, AllAround } }

#[test]
fn test_try_from() {
    assert_eq!(Get::try_from(0u8), Ok(Get::Up));
    assert_eq!(Get::try_from(1u8), Ok(Get::Down));
    assert_eq!(Get::try_from(2u8), Ok(Get::AllAround));
    assert_eq!(Get::try_from(3u8), Err(3u8));
}