pub fn main() {
    println!("3.5 제어문");

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    /********************************************/

    // else if와 다수 조건
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if number > 5 {
        println!("number > 5 조건은 참 입니다. (if 블럭)");
    } else if number < 0 {
        println!("number < 0 조건은 참 입니다. (else if 블럭)");
    } else {
        println!("모든 조건에 해당되지 않았습니다. (else 블럭)");
    }

    /********************************************/

    // let 구문에서 if 사용하기

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    // 컴파일 시에 타입이 뭔지 확실히 정의해야 한다. 아래 코드는 에러.
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };

    println!("The value of number is: {number}");

    /********************************************/

    // loop

    // 무한루프
    // loop {
    //     println!("Again!");
    // }

    // loop, if, else, break 혼합 방식
    let mut c = 5;
    loop {
        println!("c is {}", c);

        c = c - 1;

        if c < 1 {
            break; // c가 1보다 작아지면 현재 반복문 탈출
        } else {
            // 아무 구문이 없는 else 블럭을 남겨놓아도 아무 문제가 없네?
        }
    }

    // while

    let mut num = 3;

    while num != 0 { // 조건이 true인 경우 계속 반복
        println!("{}!", num);

        num = num - 1;
    }
    println!("LIFTOFF!!!");

    // 배열 a의 index를 정확히 알지 못하면 패닉 가능성 높음
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //
    //     index = index + 1;
    // }

    // for
    let a = [10, 20, 30, 40, 50];

    // 효율적인 대안 -> 콜렉션(여기서는 배열)의 각 요소를 순회하기
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 간결함
    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");

    /********************************************/

    // 연습: 화씨, 섭씨 변환
    let c = 0.0;
    let f = temperature_c_to_f(c);
    println!("{c:.0}°C = {f:.0}°F");

    let f = 212.0;
    let c = temperature_f_to_c(f);
    println!("{f:.0}°F = {c:.0}°C");

    /********************************************/

    println!("실습 완료.");
}

// 섭씨(Celsius) 온도를 화씨(Fahrenheit) 온도로 반환
fn temperature_c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// 화씨(Fahrenheit) 온도를 섭씨(Celsius) 온도로 반환
fn temperature_f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}