// extern crate bmi_calc_lib;

#[test]
fn bmi_test() {
    let result = bmi_calc_lib::bmi(180, 65);
    assert_eq!(result, Ok(20.06))
}

#[test]
fn bmi_test_invalid() {
    let result = bmi_calc_lib::bmi(0, 65);
    assert!(result.is_err());
}