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
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    // shadow sample
    println!("<shadow sample>");
    // let x = 5; // i32 일 것이라고 타입 추론.
    // warning: unused variable: `x`
    // shadow 되기 전의 x 를 사용하지 않으면 warning 발생하네..

    let x: i32 = 5; // x: i32 이라고 정확한 타입 명시함.
    let y = 10;
    println!("x = {} and y = {} and x+y = {}", x, y, x + y);
}