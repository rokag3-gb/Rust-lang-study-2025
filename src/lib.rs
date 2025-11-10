// 11장 실습

#[cfg(test)]
mod tests {
    // 11.1
    // 내부 모듈인 tests 모듈에서 외부 모듈의 코드를 테스트하려면 먼저 내부 스코프로 가져와야 함.
    // tests 모듈에서 glob (*)을 사용해서 외부 모듈에 정의된 것 전부 사용 가능.
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger)); // 논리 부정 연산자 ! 를 넣어놓아서 결과적으로 true -> 테스트 통과
    }

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn intentional_panic() {
        panic!("Make this test fail");
    }

    /*********************************************/

    // assert_eq!, assert_ne! 매크로를 이용한 동등 테스트

    // assert_eq! -> 동등 (equality) 하면 테스트 통과
    // assert_ne! -> 동등하지 않으면 (inequality) 테스트 통과

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    /*
failures:

---- tests::it_adds_two stdout ----

thread 'tests::it_adds_two' panicked at src\lib.rs:57:9:
assertion `left == right` failed
  left: 4
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrac
     */

    /*********************************************/

    // 커스텀 실패 메시지 추가하기

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));

        // 더 많은 정보를 얻을 수 있음
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result,
        );

        // failures:
        //
        // ---- tests::greeting_contains_name stdout ----
        //
        // thread 'tests::greeting_contains_name' panicked at src\lib.rs:81:9:
        // Greeting did not contain name, value was `Hello!`
    }

    /*********************************************/

    // should_panic! macro로 패닉 발생 검사하기

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 함수에서 패닉이 발생해야 하는데 (should_panic), 실제로는 패닉이 발생하지 않아서 실패했다는 의미

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }

    // failures:
    //
    // ---- tests::greater_than_100_2 stdout ----
    //
    // thread 'tests::greater_than_100_2' panicked at src\lib.rs:129:13:
    // Guess value은 100과 같거나 작아야 하는데, 200 이다.
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // note: panic did not contain expected string
    // panic message: "Guess value은 100과 같거나 작아야 하는데, 200 이다."
    // expected substring: "less than or equal to 100

    /*********************************************/

    // Result<T, E>를 이용한 테스트

    // 내부 작업이 Err를 반환할 경우 실패해야 하는 테스트를 작성하기에 편리함

    // #[test]
    // fn it_works() -> Result<(), String> {
    //     if 2 + 2 == 5 {
    //         Ok(())
    //     } else {
    //         Err(String::from("two plus two does not equal four"))
    //     }
    // }

    // failures:
    //
    // ---- tests::it_works stdout ----
    // Error: "two plus two does not equal four"

    /*********************************************/

    // 11.2 테스트 실행 방법 제어

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // 이름을 지정해 일부 테스트만 실행하기

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    #[ignore] // 테스트 무시하고 싶을 경우
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    /*********************************************/

    // 11.3 테스트 조직화

    // 테스트 모듈과 #[cfg(test)]

    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    // 비공개 함수 테스트하기

    // internal() -> 비공개 함수 internal_adder()
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    // 통합 테스트

    // tests 폴더 생성
    // tests/ch11_3_test_org.rs 생성

}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/*********************************************/

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

/*********************************************/

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value는 1~100 사이어야 하는데, {} 이다.", value);
        // }

        if value < 1 {
            panic!("Guess value은 1과 같거나 커야 하는데, {} 이다.", value);
        } else if value > 100 {
            panic!("Guess value은 100과 같거나 작아야 하는데, {} 이다.",  value);
        }

        Guess { value }
    }
}

/*********************************************/

fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    format!("Hello!")
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

/*********************************************/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}