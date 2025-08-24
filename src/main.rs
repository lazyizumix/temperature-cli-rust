use std::io;

fn main() {
    println!("変換したい値を数値で入力してね！");
    let value = get_line();

    println!("あなたが入力した値: {}", value);

    let fahrenheit = get_fahrenheit(value);
    let celsius = get_celsius(value);

    println!("摂氏 -> 華氏: {:.1}F°", fahrenheit);
    println!("華氏 -> 摂氏: {:.1}C°", celsius);
}

/*
ユーザーが入力した値をStringからF64に変換する
 */
fn get_line() -> f64 {
    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("入力エラー");

        match line.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("数値を入力してください！");
                continue;
            }
        }
    }
}

fn get_fahrenheit(value: f64) -> f64 {
    value * 1.8 + 32.0
}

fn get_celsius(value: f64) -> f64 {
    (value - 32.0) / 1.8
}
