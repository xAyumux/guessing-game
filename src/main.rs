use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!"); // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..101);

    const ANSWER_COUNT_MAX: u32 = 5;
    let mut answer_count: u32 = 0;

    loop {
        println!("Number of answer limits: {}", ANSWER_COUNT_MAX);
        answer_count += 1;
        println!("Number of answers: {}", answer_count);
        println!("Please input your guess."); // ほら、予想を入力してね

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // 次のように予想しました: {}

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),  // 小さすぎ！
            Ordering::Greater => println!("Too big!\n"), // 大きすぎ！
            Ordering::Equal => {
                println!("You win!"); // やったね！
                break;
            }
        }

        if ANSWER_COUNT_MAX <= answer_count {
            println!("Game Over!"); // ゲーム終了です！
            break;
        }
    }
}
