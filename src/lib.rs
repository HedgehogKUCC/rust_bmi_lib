pub fn bmi<T, U>(height: T, weight: U) -> Result<f64, String>
where
    T: Into<f64>,
    U: Into<f64>,
{
    let w = weight.into();
    let h = height.into() / 100.0;

    if h <= 0.0 || w <= 0.0 {
        return Err(String::from("輸入的數值有誤"));
    }

    let result_bmi = w / (h * h);
    Ok((result_bmi * 100.0).round() / 100.0)
}