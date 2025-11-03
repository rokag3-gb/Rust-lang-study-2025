pub fn main() {
    // 라이프타임이란 참조(& or &mut)가 얼마나 오래 유효한가를 컴파일러가 추적하기 위한 표시
    // 값이 살아있는 동안만 참조가 사용돼야 한다 -> 안전성 보장

    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x; // x는 블록 끝에서 drop됨
    // }
    //
    // println!("r: {}", r); // 댕글링 참조 위험

    /*****************************************************/
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// y는 독립적인 lifetime을 가짐
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}