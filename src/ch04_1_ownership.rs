use std::fmt;

pub fn main() {
    // Rust의 가장 유니크한 특성
    // Rust가 GC 없이 메모리 안정성 보장을 하게 해준다.
    // 소유권과 관련된 특성들: 빌림, 슬라이스 등

    // 스택
    // 쌓여있는 접시, 엘리베이터에서 타고 내리는 사람들
    // 스택에 접시 올리기 (pushing on the stack)
    // 스택에서 접시 치우기 (popping off the stack)

    // 코드의 어느 부분이 힙의 어떤 데이터를 사용하는지 추적하는 것,
    // 힙의 중복된 데이터의 양을 최소화하는 것,
    // 그리고 힙 내에 사용하지 않는 데이터를 제거하여 공간이 모자라지 않게 하는 것은 모두 "소유권"과 관계된 문제들

    // 변수의 스코프
    // s는 유효하지 않음. 아직 선언이 안됐다.
    let s = "hello";  // s는 이 지점부터 유효하다.
    // s를 가지고 뭔가 한다.
    // 스코프 끝. s는 더 이상 유효하지 않음.

    // fn main() {
    //     // s는 유효하지 않음. 아직 선언이 안됐다.
    //     // 변수의 스코프
    //     let s = "hello";  // s는 이 지점부터 유효함.
    //
    //     // s를 가지고 뭔가 한다.
    //
    // } // 스코프 끝. s는 더 이상 유효하지 않음.

    /**************************************************/

    let s = String::from("hello");
    // String 과 from 사이에 더블콜론(::) 은 네임스페이스 연산자

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() : 스트링에 스트링 리터럴을 붙여준다.

    println!("{s}");

    // C++에서는 이렇게 아이템의 수명주기의 끝나는 시점에 자원을 해제하는 패턴을 종종,
    // 자원 습득이 곧 초기화 (Resource Acquisition Is Initialization, RAII) 라고 부른다.
    // 러스트의 drop 함수는 여러분이 RAII 패턴을 경험해본 적 있다면 익숙할 것.

    let s1 = String::from("hello"); // s1의 포인터는 "hello" 스트링 리터럴의 주소

    let s2 = s1; // s1 value "moved" to s2 (not copy)

    // println!("{}, world!", s1); // 에러!

    /**************************************************/

    // 변수와 데이터가 상호작용하는 방법: 클론
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 힙 데이터가 복사되는 동작을 명시하면

    println!("s1 = {}, s2 = {}", s1, s2); // 둘다 출력 가능

    // 스택에만 있는 데이터: 복사
    let x = 5;
    let y = x; // clone() 안함.

    println!("x = {}, y = {}", x, y); // 그런데도 잘 출력된다?

    // 정수형과 같이 컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장되기 때문에, 실제 값의 복사본이 빠르게 만들어질 수 있다.
    // = y가 생성된 후에 x가 더 이상 유효하지 않도록 해야할 이유가 없다는 뜻.

    // 방금 전 x와 y 처럼 i32은 copy가 된다.
    // 이유 = "컴파일 타임에 결정되어 있는 크기의 타입은 스택에 저장되고 실제 값 복사가 빠르기 때문"

    // Copy 가능한 타입: 정수형, bool, 부동 소수점, Copy 가능한 타입들로만 이루어진 Tuple

    /**************************************************/

    // 소유권과 함수

    let s = String::from("hello"); // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);                     // s의 값이 함수 안으로 이동했습니다.
    // ... 그리고 이제 더이상 유효하지 않습니다.

    // takes_ownership(s.clone()); // 이렇게 복제하지 않는 이상..

    // println!("s = {s}"); // 에러 "값이 이동된 후에 사용되었습니다"

    let x = 5;                          // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                          // x가 함수 안으로 이동했습니다만,
    // i32는 Copy가 되므로 (Copy 가능한 타입이기 때문에)
    // x를 이후에 계속 사용해도 됩니다.

    println!("x = {x}"); // 정상

    let c = 'a';
    makes_copy2(c);
    println!("c = {c}"); // 정상. 그렇다면 의문이 드는데,
    // 질문: char 도 "컴파일 타임에 결정되어 있는 크기의 타입은 스택에 저장되고 실제 값 복사가 빠르기 때문" 에 copy 되는거임?

    makes_copy2(2139812171);
    makes_copy2('👉');
    makes_copy2("1ddasd");
    makes_copy2(654.1);
    makes_copy2(true);

    /**************************************************/

    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
    // 이동시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
    // 이동되었고, 이 함수가 반환값을 s3으로도
    // 이동시켰습니다.

    // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3); // s2는 이동되었다는 에러 (value borrowed here after move)

    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3); // s1 복제해서 s2에 소유권 주면 이제 에러 안남.

    /**************************************************/

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

fn makes_copy_char(some_char: char) { // some_char 가 스코프 안으로 들어왔습니다.
    println!("{}", some_char);
}

fn makes_copy2<T: std::fmt::Display>(some: T) { // some_char 가 스코프 안으로 들어왔습니다.
    println!("{}", some);
}

/**************************************************/

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
    // 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string                              // some_string이 반환되고, 호출한 쪽의
    // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
    // 안으로 들어왔습니다.

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

/**************************************************/

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 불필요한 return 문
}