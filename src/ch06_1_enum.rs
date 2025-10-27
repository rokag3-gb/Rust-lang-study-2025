pub fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrOld {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrOld {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 열거형의 각 variant에 직접 데이터를 치환함.
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);
    // [src\ch06_1_enum.rs:23:5] home = V4(
    //     127,
    //     0,
    //     0,
    //     1,
    // )

    /***************************************************/

    let m = Message::Write(String::from("hello"));

    m.call(); // 열거형 안에 구현체 함수 call() 을 호출함.

    /***************************************************/

    // Option 열거형이 널 값보다 좋은 점들

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // Option<T> 와 T 는 다른 형식이다.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`

    // Option<T>의 여러 메서드를 익혀두면 앞으로의 러스트 프로그래밍에 매우 많은 도움이 될 것.

}

enum Message { // Message 열거형은 각 배리언트가 다른 타입과 값을 가진다.
    Quit, // 아무 데이터 없음
    Move { x: i32, y: i32 }, // struct 처럼 필드이름과 형식을 가짐.
    Write(String), // 하나의 String을 가짐.
    ChangeColor(i32, i32, i32), // 3개의 i32을 가짐.
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit 메시지로 호출됨."),
            Message::Move { x, y } => println!("Move 메시지로 호출됨. x = {}, y = {}", x, y),
            Message::Write(t) => println!("Write 메시지로 호출됨. (String = {})", t),
            Message::ChangeColor(r, g, b) => println!("ChangeColor 메시지로 호출됨. RGB({}, {}, {})", r, g, b),
        }
    }
}

// 이렇게 4개의 구조체로 각각 따로 선언할 수 도 있지만 위에 열거형을 사용하는 방식이 훨씬 심플하다.
// struct QuitMessage; // Unit Struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // Tuple Struct
// struct ChangeColorMessage(i32, i32, i32); // Tuple Struct

/***************************************************/

fn route(ip_kind: IpAddrKind) {
    //
}

enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // 열거형 variant
    V6(String),
}

struct IpAddrOld {
    kind: IpAddrKind,
    address: String,
}