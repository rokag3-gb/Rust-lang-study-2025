extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn main() {
    println!("Guess the number!");

    // thread_rng() -> OS가 시드(seed)를 정하고 현재 스레드에서만 사용되는 특별한 정수생성기
    // 1부터 100 사이의 숫자를 랜덤 생성

    let secret_number = rand::thread_rng().gen_range(1, 101); // 이상 ~ 미만
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // 러스트에서 모든 변수는 기본적으로 불변(immutable)

        let mut guess = String::new(); // 사용자의 입력값을 저장할 공간을 생성. 가변(mutable)

        // & -> 참조자
        // let res = io::stdin().read_line(&mut guess);
        // res.expect("Failed to read line");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 이전에 있었던 guess: String을 가리는 것을 허용한다. (shadow)
        // String -> u32 으로 정확한 타입을 명시
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //.expect("Please type a number!");

        println!("You guessed. [{}] {}", guess, "!!!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 큽니다. 좀 낮추세요!"),
            Ordering::Greater => println!("너무 작아요. 좀 올리세요!"),
            Ordering::Equal => {
                println!("You win! 숫자를 맞추셨습니다!");
                break;
            },
        }
    }
}