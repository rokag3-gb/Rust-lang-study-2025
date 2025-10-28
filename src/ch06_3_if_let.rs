pub fn main() {
    // if let을 사용한 간결한 제어 흐름
    let config_max = Some(3u8);
    
    // Some, None 구조
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    // if let은 한 패턴에 매칭될 때만 코드를 실행하고 다른 경우는 무시하는 match 문을 작성할 때 사용하는 문법 설탕 (syntax sugar)
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    /*******************************************************/

    let coin = Coin::Quarter(UsState::California);

    // 만일 쿼터가 아닌 모든 동전을 세고 싶은 동시에 쿼터 동전일 경우도 알려주고 싶다면 아래와 같이 match문을 쓸 수도 있음.
    let mut count = 0;
    
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?} !", state),
    //     _ => count += 1,
    // }

    // if let 과 else 활용
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?} !", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum UsState { Alabama, Alaska, California }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    // fn value_in_cents(&self) -> u8 {
    //     match self {
    //         Coin::Penny => {
    //             println!("행운의 페니!");
    //             1 // print 후 1 리턴
    //         }
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         // Coin::Quarter => 25,
    //         Coin::Quarter(s) => {
    //             println!("State quarter from {:?}!", s);
    //             25
    //         },
    //     }
    // }

    // let mut count = 0;

    // fn value_count(&self) -> u8 {
    //     match self {
    //         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //         _ => count += 1, // 모든 동전을 세고 싶은 동시에
    //     }
    // }
}