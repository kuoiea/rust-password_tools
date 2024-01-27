use std::io::stdin;

use rand;
use rand::Rng;

#[derive(Debug)]
struct Password {
    password: String,
}


impl Password {
    // 初始化密码结构体
    fn new() -> Password {
        return Password {
            password: String::new()
        };
    }

    fn create_password(&mut self, password_level: String) {
        let password_length;
        let special_str;
        match password_level.trim() {
            "L" => {
                password_length = 10;
                special_str = false
            }
            "M" => {
                password_length = 13;
                special_str = true
            }
            "H" => {
                password_length = 16;
                special_str = true
            }
            _ => {
                password_length = 8;
                special_str = false
            }
        };

        let random_str: String = create_random_str(password_length, special_str);

        self.password = random_str
    }
}


/// 这个方法用来生成一个长度的随机字符串
///
/// # Params
/// - `password_length`：一个数字，用来表示密码生成的长度。
/// - `special_str`: 布尔值， 用来限制是否使用特殊字符
///
/// # Return
///  String 生成的随机字符串
fn create_random_str(password_length: u8, special_str: bool) -> String {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let special_str_srt: &[u8] = b"!@#$%^&*()_+-=[]{}|;':,.<>?/\\\"`~";

    let combined_vec: Vec<u8>;
    if special_str {
        combined_vec = [charset, special_str_srt].concat();
    } else {
        combined_vec = charset.to_vec();
    }


    let mut rng = rand::thread_rng();
    let random_string: String = (0..password_length)
        .map(|_| {
            let index = rng.gen_range(0..combined_vec.len());
            combined_vec[index] as char
        })
        .collect();

    return random_string;
}


fn main() {
    println!("生成随机密码，请输入对应的密码等级字母： L(低级), M(中级), H(高级)");
    let mut user_input: String = String::new();
    let stdin = stdin();
    let _ = stdin.read_line(&mut user_input);
    let mut password = Password::new();
    password.create_password(user_input);

    println!("{:?}", password.password)
}
