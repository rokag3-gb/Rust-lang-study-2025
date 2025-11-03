use std::fs;
use std::fs::File;
use std::io::ErrorKind; // 이런 직관적인 이름이라니..
use std::io::{self, Read};
use std::net::IpAddr;

pub fn main() {
    // panic!("crash and burn"); // 난리 남. unwinding 시작.

    let v = vec![1, 2, 3];
    // v[99];

    // thread 'main' panicked at src\ch09_error_handling.rs:6:6:
    // index out of bounds: the len is 3 but the index is 99
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\Rust-lang-study-2025.exe` (exit code: 101)

    // RUST_BACKTRACE 출력
    // Mac: RUST_BACKTRACE=1 cargo run
    // Win: $env:RUST_BACKTRACE=1
    // Win: cargo run

    /*************************************************************/

    // Result로 복구 가능한 에러 처리하기

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // match는 유용하지만 원시적이기도 함.

    // unwrap_or_else 메서드와 클로저를 사용한 버전
    // let greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
    //     ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => panic!("Problem creating the file: {:?}", e),
    //     },
    //     other_error => {
    //         panic!("Problem opening the file: {:?}", other_error);
    //     }
    // });

    /*************************************************************/

    // 에러 발생 시 패닉을 위한 숏컷: unwrap과 expect

    // 만일 Result가 Err variant라면 unwrap은 panic! 매크로를 호출
    // let greeting_file = File::open("hello.txt")
    //     .unwrap();

    // unwrap 대신 expect를 이용하여 좋은 에러 메시지를 제공하면, 의도를 전달하면서 패닉의 근원을 추적하는 걸 도와줌
    // unwrap: panic!의 기본 메시지 출력
    // expect: 내가 직접 매개변수로 전달한 에러 메시지 출력
    // let greeting_file = File::open("hello.txt")
    //     .expect("이 프로젝트 안에 hello.txt 이 없어!");

    // 프로덕션 환경에서 unwrap보다 expect를 선택하여 더 많은 맥락 제공

    /*************************************************************/

    // 에러 전파하기
    match read_username_from_file() {
        Ok(name) => println!("username: {}", name),
        Err(e) => eprintln!("에러 발생: {:?}", e),
    }

    // 에러를 전파하기 위한 숏컷: ?
    let username2 = read_username_from_file2();

    // 에러! fn main() 은 반환 타입이 Result가 아니라 ()
    // cannot use the `?` operator in a function that returns `()`
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    // let greeting_file = File::open("hello.txt")?;

    // 첫번째 행의 마지막 글자
    let text1 = "Hello world\nThis is Rust";
    let text2 = "Rust\nLanguage";
    let text3 = "";

    println!("last_char_of_first_line(text1) = {:?}", last_char_of_first_line(text1));
    println!("last_char_of_first_line(text2) = {:?}", last_char_of_first_line(text2));
    println!("last_char_of_first_line(text3) = {:?}", last_char_of_first_line(text3));

    /*************************************************************/

    // 여러분이 컴파일러보다 더 많은 정보를 가지고 있을 때

    // 코드를 조사하여 Err variant가 나올리 없음을 확신할 수 있다면 unwrap을 호출해도 아무 문제 없음
    // expect의 문구에 Err variant 관련된 내용을 적어주면 좋음
    // 이 문자열이 항상 유효한 IP 주소라는 사실을 컴파일러는 알 수 없음 (나는 알지)
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // 나쁜 상태에 처하게 될 가능성이 있을 때는 코드에 panic!을 넣는 것이 바람직
    // 나쁜 상태 = 예기치 못한 무언가, 터프한 입력 등등
}

/****************************************************************************************/

use std::error::Error;

fn main2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
    // Box<dyn Error> 트레잇 객체는 '어떠한 종류의 에러'
    // 17장 "트레이트 객체를 사용하여 다른 타입의 값 허용하기"에서 다룰 예정
}

/****************************************************************************************/

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? -> Result가 Ok면 그 안의 값을 꺼내서 계속 진행하고, Err면 그 에러를 현재 함수에서 즉시 return
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut name = String::new();
    username_file.read_to_string(&mut name)?;
    Ok(name)
}

// 더 간결하게
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut name = String::new();
    File::open("hello.txt")?.read_to_string(&mut name)?;
    Ok(name)
}

// 더더 간결하게
fn read_username_from_file4() -> Result<String, io::Error> {
    // read_to_string 자체가 Result<String, io::Error>를 반환하니까 그대로 반환해줌
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines() // 해당 문자열의 라인에 대한 반복자
        .next()? // next()를 호출하여 첫번째 값을 얻어오기. text가 비어있다면 ? 를 사용하여 함수 실행을 멈추고 None 반환
        .chars()
        .last()
}