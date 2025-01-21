// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
   /*???("blue");
    ???("red".to_string());
    ???(String::from("hi"));
    ???("rust is fun!".to_owned());
    ???("nice weather".into());
    ???(format!("Interpolation {}", "Station"));
    ???(&String::from("abc")[0..1]);
    ???("  hello there ".trim());
    ???("Happy Monday!".to_string().replace("Mon", "Tues"));
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase());*/
        string_slice("blue");  // 字符串字面值是 &str 类型
        string("red".to_string());  // to_string() 生成 String 类型
        string(String::from("hi"));  // String::from 生成 String 类型
        string("rust is fun!".to_owned());  // to_owned() 生成 String 类型
        string("nice weather".into());  // into() 自动推断为 String 类型
        string(format!("Interpolation {}", "Station"));  // format! 生成 String 类型
        string_slice(&String::from("abc")[0..1]);  // 字符串切片，需要 &str
        string_slice("  hello there ".trim());  // trim() 返回 &str
        string("Happy Monday!".to_string().replace("Mon", "Tues"));  // replace() 生成 String 类型
        string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // to_lowercase() 生成 String 类型
    
}
