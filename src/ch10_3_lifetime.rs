pub fn main() {
    // 라이프타임이란 참조(& or &mut)가 얼마나 오래 유효한가를 컴파일러가 추적하기 위한 표시
    // 값이 살아있는 동안만 참조가 사용돼야 한다 -> 안전성 보장

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let evens = filter_even(&nums);

    println!("짝수만 필터링됨: {:?}", evens);

    // let r;                // ---------+-- 'a
    //                       //          |
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |  // x는 블록 끝에서 drop됨
    // }                     // -+       |
    //                       //          |
    // println!("r: {}", r); //          |  // 댕글링 참조 위험
}                            // ---------+

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// y는 독립적인 lifetime을 가짐
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn filter_even(numbers: &Vec<i32>) -> Vec<i32> {
    numbers
        .iter()                        // 벡터의 요소를 순회하는 이터레이터 생성
        .filter(|&n| n % 2 == 0) // 짝수만 통과시키는 조건
        .cloned()                       // 참조(&i32) -> 값(i32)으로 복사
        .collect()                                        // Vec<i32>로 다시 수집
}