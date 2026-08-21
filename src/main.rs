use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// 讀取使用者輸入並轉換成 [`u32`]。
///
/// 如果使用者輸入的內容無法轉換成數字，
/// 就會要求使用者重新輸入。
///
/// # Returns
///
/// 回傳使用者輸入的數字。
///
/// # Example
///
/// ```
/// let number = read_number();
/// ```
fn read_number() -> u32 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("讀取該行失敗");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("請輸入有效的數字！");
            }
        }
    }
}

/// 讀取使用者輸入的最小值與最大值。
///
/// 如果最小值大於或等於最大值，
/// 就會要求使用者重新輸入。
///
/// # Returns
///
/// 回傳 `(min, max)`，其中：
/// - `min` 是最小值
/// - `max` 是最大值
///
/// # Example
///
/// ```
/// let (min, max) = read_range();
/// ```
fn read_range() -> (u32, u32) {
    loop {
        println!("請輸入最小數字：");
        let min = read_number();

        println!("請輸入最大數字：");
        let max = read_number();

        if min >= max {
            println!("最小數字必須小於最大數字，請重新輸入！");
            continue;
        }

        break (min, max);
    }
}

/// 執行猜數字遊戲。
///
/// 持續要求使用者輸入猜測數字，並與指定的答案比較。
/// 根據比較結果提示使用者猜測的數字太小、太大或猜測正確。
///
/// # Arguments
///
/// * `secret_number` - 遊戲中預先產生的正確答案。
///
/// # Example
///
/// ```
/// let secret_number = 50;
/// guess_number(secret_number);
/// ```
fn guess_number(secret_number: u32) {
    loop {
        println!("請輸入你的猜測數字。");

        let guess = read_number();

        println!("你的猜測數字：{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("獲勝！");
                break;
            }
        }
    }
}

/// 猜數字遊戲的程式進入點。
///
/// 先取得使用者指定的數字範圍，
/// 再從該範圍中產生隨機答案，
/// 最後開始猜數字遊戲。
fn main() {
    let (min, max) = read_range();

    println!("請猜測一個 {min}～{max} 的數字！");

    let secret_number = rand::thread_rng().gen_range(min..=max);

    guess_number(secret_number);
}
