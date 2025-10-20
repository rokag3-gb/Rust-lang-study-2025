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

    let mut s = String::from("hello");

    let r1 = &s; // 불변 참조자
    let r2 = &s; // 불변 참조자
    let r3 = &mut s; // 스코프 내에서 불변 참조자를 가지고 있을 동안에 가변 참조자 역시 만들 수 없다.

    /******************************************************/

    // Dangling References

    // 댕글링 포인터란? 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을지도 모를 메모리를 참조하고 있는 포인터

    // let reference_to_nothing = dangle();

    // fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다
    //     let s = String::from("hello"); // s는 새로운 String입니다
    // 
    //     &s // 우리는 String s의 참조자를 반환합니다.
    // } // 여기서 s는 스코프를 벗어나고 버려집니다. 이것의 메모리는 사라집니다.
    // // 위험하군요!

    // 풀이: s가 dangle() 안에서 만들어졌고 dangle() 스코프가 끝나면 s는 할당 해제된다. 근데 &s (s의 참조자)를 반환하려고 했기 때문에 문제이다!
    // 이 참조자 &s가 어떤 무효화된 String을 가리키게 될 것이라는 의미잖아. (= 댕글링 참조될 뻔)
    // Rust는 이런 것을 미연에 방지해준다.
    
    let no_dangle = no_dangle();

    /******************************************************/
    
    // 결론: 하나의 스코프 안에서 다음 둘 중 하나만 가질 수 있다.
    // 1) 하나의 가변 참조자,
    // 2) 임의 갯수의 불변 참조자들
    
    // 참조자는 항상 유효해야 함 (댕글링 원천 차단)
    
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

fn no_dangle() -> String { // 댕글링 참조자가 발생하지 않도록 s를 직접 반환하면 된다.
    let s = String::from("hello");
    s
}