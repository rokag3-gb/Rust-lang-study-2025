pub use crate::front_of_house::hosting;

fn main() {
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    add_to_waitlist();
}

mod example_7_15 {
    // 예제 7-15: 이름이 같은 두 개의 타입을 동일한 스코프에 가져오려면 부모 모듈을 반드시 명시해야 합니다.
    use std::fmt;
    use std::io;
    fn function1() -> fmt::Result { Ok(()) }
    fn function2() -> io::Result<()> { Ok(()) }

    // 예제 7-16: as 키워드로 새로운 이름 제공하기
    use std::fmt::Result;
    use std::io::Result as IoResult; // 충돌을 방지하기 위하여 이름을 새롭게 지정
    fn function3() -> Result { Ok(()) }
    fn function4() -> IoResult<()> { Ok(()) }
}

/**********************************************/

// 중첩 경로를 사용하여 대량의 use 나열을 정리하기
// As-is
use std::cmp::Ordering;
use std::io;
// To-be
use std::{cmp::Ordering, io};

// As-is
use std::io;
use std::io::Write;
// To-be
use std::io::{self, Write};

// 글롭 연산자 (*)
// 경로 안에 정의된 모든 공개 아이템을 가져올 수 있음.
use std::collections::*;
// 사용에 주의