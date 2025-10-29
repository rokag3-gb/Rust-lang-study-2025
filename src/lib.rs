// mod ch07_4_use_keyword;

use std::{fmt, io};

pub mod front_of_house {
    // 모듈을 공개했다고 해서 내용까지 공개되는 것은 아니다.
    // 모듈은 단순한 컨테이너이기 때문에, 모듈을 공개하는 것 만으로 할 수 있는 것은 별로 없음.
    // 여기에 더해, 모듈이 가지고 있는 아이템도 마찬가지로 공개해야 함.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_over() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order () {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 열거형은 공개로 지정할 경우 모든 variant가 공개됨.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        // super로 시작하면 현재 모듈 혹은 크레이트 루트 대신 자기 부모 모듈부터 시작되는 상대 경로를 만들 수 있음.
        // 여기서 super는 back_of_house 의 부모 = 크레이트 루트 = src/lib.rs
        // 그렇기 때문에 현재 컨택스트에서 fn deliver_order() 을 호출할 수 있는 것.
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 호밀(Rye) 토스트를 곁을인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 먹고 싶은 빵 "Wheat" 으로 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // 다음 라인의 주석을 해제하면 컴파일되지 않는다.
    // 식사와 함께 제공되는 계절 과일은 조회나 수정이 허용되지 않는다. (seasonal_fruit 필드가 pub이 아님)
    // meal.seasonal_fruit = String::from("blueberries");

    // 열거형은 공개로 지정할 경우 모든 variant가 공개됨.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist(); // crate:: -> 경로 접두사는 불필요함.
    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}

// use가 사용된 특정 스코프에서만 유효함.
// use crate::front_of_house::hosting;
// use를 customer 모듈 안으로 이동시키니까 에러없어짐.
mod customer {
    pub use crate::front_of_house::hosting; // <- customer 안으로 이동
    
    // pub use: 다시 내보내기(re-exporting)
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// 7.5. 별개의 파일로 모듈 분리하기
mod ch07_front_of_house;
pub use crate::ch07_front_of_house::front_of_house::hosting;
pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}