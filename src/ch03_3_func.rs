fn another_function() {
    println!("another_function. snake_case를 사용합니다.");
}

pub fn main() {
    println!("3.3 함수 동작 원리 입니다.");

    another_function();
    another_function2(123, 456);

    // 구문(statements)은 값을 반환하지 않음.
    // 에러 발생! let 구문을 사용해서 x 변수에 값을 치환할 수 없음.
    // let x = (let y = 6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 세미콜론으로 끝나지 않은 점 주목!
        // 만약 세미콜론을 '표현식' 마지막에 추가하면, 이는 '구문'으로 변경되고 반환 값이 아니게 됨.
    };

    println!("The value of y is: {y}");

    println!("five() 함수의 반환 값 = {}", five());

    let x = plus_one(5);
    println!("plus_one 함수 호출 반환값 x = {x}");
}

fn another_function2(x: i32, y: i32) {
    println!("another_function2. The value of x is: {}", x);
    println!("another_function2. The value of y is: {}", y);
}

// 반환 값을 갖는 함수는 "->" 뒤에 타입을 선언
fn five() -> i32 {
    5
    // return 5 // 정상 작동함.
    // return 5; // 정상 작동함.
}

fn plus_one(x: i32) -> i32 {
    x + 1
}