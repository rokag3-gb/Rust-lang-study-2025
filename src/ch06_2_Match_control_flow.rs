pub fn main() {
    let coin = Coin::Quarter(UsState::California);

    println!("Value is {} cents.", coin.value_in_cents());
    println!();

    /*********************************************************/

    // Option<T>를 이용하는 매칭

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five = {:?}\nsix = {:?}\nnone = {:?}", five, six, none);
    println!();

    // Rust의 match는 철저하다 (exhaustive)
    // 발생할 수 있는 모든 경우 중 어떤 패턴을 놓쳤는지 알려줌.
    // 따라서 유효한 코드를 만들려면 모든 가능성을 샅샅이 다루어야 함. -> 엄격함.

    /*********************************************************/

    // 포괄 패턴(Catch-all pattern) 과 _ (underscore) placeholder

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(dice_roll),
        7 => remove_fancy_hat(dice_roll),
        // other => move_player(other), // Catch-all pattern
        // _ => move_player(dice_roll), // _ = 위에서 나열하지 않은 모든 값에 대해 마지막으로 패턴 매칭됨.
        _ => (), // 패턴 매칭은 되는데, 아무 일도 일어나지 않음.
    }
}

fn add_fancy_hat(num: u8) {
    println!("{} 이면 add_fancy_hat 호출.", num);
}
fn remove_fancy_hat(num: u8) {
    println!("{} 이면 remove_fancy_hat 호출.", num);
}
fn move_player(num: u8) {
    println!("{} 이면 move_player 호출.", num);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState { Alabama, Alaska, California }

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Quarter,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("행운의 페니!");
                1 // print 후 1 리턴
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // Coin::Quarter => 25,
            Coin::Quarter(s) => {
                println!("State quarter from {:?}!", s);
                25
            },
        }
    }

    // fn value_count(&self) -> u8 {
    //     let mut count = 0;
    //     match self {
    //         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //         _ => count += 1, // 모든 동전을 세고 싶은 동시에
    //     }
    // }
}