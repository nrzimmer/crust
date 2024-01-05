use lazy_static::lazy_static;

lazy_static! {
    static ref NAME: String = env!("CARGO_PKG_NAME").to_string();
    static ref NAME_CAPITALIZED: String = {
        let s= env!("CARGO_PKG_NAME");
        format!("{}{}", s.chars().next().unwrap().to_uppercase(), s.chars().skip(1).collect::<String>())
    };
}

#[allow(non_snake_case)]
pub fn Name() -> &'static str {
    NAME_CAPITALIZED.as_str()
}

pub fn name() -> &'static str {
    NAME.as_str()
}