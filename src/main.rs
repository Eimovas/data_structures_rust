use crate::string_builder::StringBuilder;

mod string_builder;
mod stack;
mod queue;

fn main() {
    let mut sb = StringBuilder::new();
    sb.append(String::from("Algis"));
    sb.append(String::from(" "));
    sb.append(String::from("nori"));
    sb.append(String::from(" "));
    sb.append(String::from("valgyt"));

    let str = sb.to_string();
    println!("{}", str);
}
