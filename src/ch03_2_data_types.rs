use std::mem::size_of;

pub fn main() {
    // Rust는 모든 변수의 타입이 컴파일 전에 미리 정해져 있어야 함.

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess = {guess}");

    /**********************************************/

    // 스칼라(Scalar) 타입: 정수형, 부동 소수점, Boolean, 문자
    // 복합(Compound) 타입: Tuple, Array

    // 정수형 signed
    let a: i8 = 127;   // (8비트) -128 ~ 127
    let a: i16 = 32_767;  // (16비트) -32,768 ~ 32,767. Short.
    // 타입 생략 시 i32
    let a = 2_147_483_647;   // approx. -21억 4748만 ~ 21억 4748만. Int.
    let a: i64 = 9_223_372_036_854_775_807;  // approx. -9경 2233조 ~ 9경 2233조. Long.
    let a: i128 = 17 * 10i128.pow(37); // approx. -1.7e38 ~ 1.7e38. BigInteger.

    // isize, usize 는 64-bit 아키텍처이면 64bit를, 32-bit 아키텍처이면 32bit를 갖음.

    // 정수형 unsigned
    let a: u8 = 255;  // max 255.
    let a: u16 = 65_535; // max 65,535. UShort.
    let a: u32 = 4_294_967_295; // max approx. 42억9496만. UInt.
    let a: u64 = 18_446_744_073_709_551_615; // max approx. 1경 8446조. ULong.
    let a: u128 = 34 * 10u128.pow(37); // max approx. 3.4e38.

    println!("i32: min = {}, max = {}, bytes = {}", i32::MIN, i32::MAX, size_of::<i32>());
    println!("i64: min = {}, max = {}, bytes = {}", i64::MIN, i64::MAX, size_of::<i64>());

    println!("u32: min = {}, max = {}, bytes = {}", u32::MIN, u32::MAX, size_of::<u32>());
    println!("u64: min = {}, max = {}, bytes = {}", u64::MIN, u64::MAX, size_of::<u64>());

    // 부동 소수점 타입
    // IEEE-754
    let x = 2.0; // float default type: f64 (64비트)
    let y: f32 = 3.0; // f32 명시 (32비트)

    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder

    // Boolean 타입
    let t = true;
    let t: bool = false;
    println!("bool t is {}", t);

    // 문자 타입
    let c = 'z'; // char = Unicode Scalar
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /**********************************************/

    // 복합(Compound) 타입

    // Tuple 타입
    let tup: (char, u16) = ('넥', 65_535);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // Tuple 구조 해체

    println!("The value of y in tuple = {y}");

    let five_hundred = tup.0; // tup의 0번째 색인 값
    let six_point_four = tup.1; // tup의 1번째 색인 값
    let one = tup.2; // tup의 2번째 색인 값

    // Array 타입
    // (stack에 할당됨)
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // 배열 요소에 접근
    let first = a[0];
    let second = a[1];

    let index = 2;
    let element = a[index]; //  index out of bounds: the length is 5 but the index is 10
    // cargo check에서는 error 발생 없음
    // cargo build, run 에서는 error 발생
    println!("The value of element is: {}", element);

    // 오류와 함께 종료 될 때 -> 패닉(panic) 이라고 표현
}