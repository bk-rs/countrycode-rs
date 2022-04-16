#![cfg(feature = "once_cell")]

use countrycode::RECORDS_ISO2_MAP;

#[test]
fn test_static() {
    let record = RECORDS_ISO2_MAP.get("US").unwrap();
    println!("{:?}", record);
    assert_eq!(record.iso3, "USA".into());
}
