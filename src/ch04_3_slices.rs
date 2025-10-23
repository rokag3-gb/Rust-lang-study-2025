pub fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    // word는 5가 될 것임.

    // s.clear(); // String을 비워서 ""로 만듦.
    // word는 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 String은 ""으로 변해버렸음.

    println!("the first word is: {word}");

    /**********************************************/

    // String Slice

    let s = String::from("hello world");

    let hello = &s[0..5]; // String 일부분에 대한 참조자
    let world = &s[6..11]; // String 일부분에 대한 참조자

    println!("hello {hello}, world {world}!");

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; // .. 앞 숫자를 생략하면 제일 앞부터.

    let slice = &s[3..s.len()];
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]; // .. 끝 숫자를 생략하면 제일 끝까지.

    let slice = &s[0..len];
    let slice = &s[..]; // .. 앞 숫자, 끝 숫자를 모두 생략하면 슬라이스 전체임.

    /**********************************************/

    let my_string = String::from("hello world");

    // first_word가 'String'의 슬라이스로 동작한다.
    let word = first_word(&my_string[..]);

    let my_string_literel = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작한다.
    let world = first_word(&my_string_literel[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 떄문에, 아래 코드도 슬라이스 문법 없이 동작한다!!
    let world = first_word(my_string_literel);

    /**********************************************/

    // 그 밖의 슬라이스들
    let a = [7, 1, 2, 3, 4, 5, -120];
    let slice = &a[0..3]; // reference by range (범위 참조)

    for element in slice {
        println!("값은 {} !", element);
    }
    println!();

    for value in slice.iter() {
        println!("slice value = {}", value);
    }
    println!();

    for (idx, value) in slice.iter().enumerate() {
        println!("slice[{}] = {}", idx, value);
    }
    println!();

    println!("4.3. Slices 끝!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // 바이트 배열의 반복자(iterator)를 생성
    for (i, &item) in bytes.iter().enumerate() { // enumerate 메소드는 튜플을 반환함
        if item == b' ' { // '공백 리터럴'과 값을 비교
            return &s[0..i];
        }
    }

    &s[..] // 공백이 없으면 전체 문자열
}