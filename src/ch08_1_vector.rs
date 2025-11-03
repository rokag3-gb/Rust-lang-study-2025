pub fn main() {
    println!("8.1. 벡터에 여러 값의 목록 저장하기");

    let v: Vec<i32> = Vec::new();

    // vec! 매크로 활용
    let v = vec![1, 2, 3]; // v의 타입은 Vec<i32>일 것으로 추정 가능

    let mut v = Vec::new();
    // Vec<i32> 라고 타입을 명시하지 않아도 되는 이유는 push한 값이 모두 i32 타입이라 Rust가 v의 타입을 Vec<i32>이라고 추론했기 때문.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /********************************************/

    // 벡터 요소 읽기

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("세번째 값을 {} 이다.", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // panic 발생! 존재하지 않는 요소를 참조하기 때문.
    let does_not_exist = v.get(100); // 벡터 범위를 벗어난 인덱스가 주어지면 panic 없이 None이 반환됨.

    println!("does_not_exist = {:?}", does_not_exist);
    // "does_not_exist = None" 출력됨.

    // 8-6: 아이템의 참조자를 가지고 있는 상태에서 벡터에 새로운 요소 추가 시도하기
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("The first element is {first}");

    /********************************************/

    // 벡터 값에 대해 반복하기

    // 8-7: for 루프로 벡터의 요소들에 대해 반복하여 각 요소를 출력하기
    let v = vec![100, 30, 50];
    for i in &v {
        println!("v[i] = {i}");
    }

    // 8-8: 벡터의 요소에 대한 가변 참조자로 반복하기
    let mut v = vec![100, 30, 50];
    for i in &mut v {
        // dereference operator (역참조 연산자)
        // * 역참조 연산자로 i의 값을 얻어내야 함.
        *i = *i + 7;
        // *i += 7; // (동일)
    }
    for i in &v {
        println!("v[i] = {i}");
    }

    /********************************************/

    // 열거형을 이용해 여러 타입 저장하기

    // 8-9: 열거형을 정의하여 벡터 내에 다른 타입의 데이터를 담을 수 있도록 하기
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    // v, row를 가지고 작업하기
} // <- 여기서 v, row 등이 스코프 밖으로 벗어나고 해제됨 (drop)