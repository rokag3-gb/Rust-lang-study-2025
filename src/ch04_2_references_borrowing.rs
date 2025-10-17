pub fn main() {
    // 참조자 (References)
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // & 는 참조자

    println!("The length of '{s1}' is {len}.");

    /******************************************************/

    let s = String::from("hello");
    // change(&s);

    /******************************************************/

    // 가변 참조자 (Mutable References)

    let mut s = String::from("hello");

    change(&mut s);

    println!("가변 String인 s의 값은 {s}");

    let mut s = String::from("hello");

    // 가변 참조자의 딱 한가지 큰 제한 -> 특정 스코프 내에 특정한 데이터에 대한 가변 참조자는 "딱 하나만 만들 수 있다"는 것.

    let r1 = &mut s;
    // let r2 = &mut s; // 에러 발생! "second mutable borrow occurs here"
    // println!("r1 = {r1}, r2 = {r2}");

    // Rust는 가변 참조자를 매우 통제된 형식으로 허용함.
    // 이점은 컴파일 타임에 데이터 레이스(data race)를 방지할 수 있도록 해준다는 것. (race condition)

    let mut s = String::from("hello");
    {
        // 새로운 스코프를 만들기 위해 중괄호를 사용하곤 하는데, 이런 식으로 여러 개의 가변 참조자를 만들 수 있음.
        // (눈 가리고 아웅이네ㅋㅋ)
        let r1 = &mut s;
    }
    let r2 = &mut s;
    
    println!("r1 = {r1}, r2 = {r2}");
    
    /******************************************************/
}

fn calculate_length(s: &String) -> usize { // s는 String의 참조자입니다.
    // 빌림 (Borrow): 함수의 파라미터로 참조자를 만드는 것
    // s: &String
    s.len()
} // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기 때문에, 아무런 일도 발생하지 않습니다.

fn change(some_string: &mut String) {
    // 변수가 기본적으로 불변인 것처럼, 참조자도 마찬가지. 우리가 참조하는 어떤 것을 변경하는 것은 허용되지 않는다.
    some_string.push_str(", world");
}