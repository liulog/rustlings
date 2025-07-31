// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz"); // longest 将输入中较短的生命周期作为返回引用的生命周期
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}
