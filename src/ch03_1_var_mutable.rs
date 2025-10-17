pub fn main() {
    println!("<변수와 가변성>");

    let mut x = 5; // 가변으로 선언하면 값 변경 가능
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
    
    /*************************************************************/
    
    // Shadowing sample
    println!("<shadow sample>");
    // let x = 5; // i32 일 것이라고 타입 추론.
    // warning: unused variable: `x`
    // shadow 되기 전의 x 를 사용하지 않으면 warning 발생하네..

    let x: i32 = 2; // x: i32 이라고 정확한 타입 명시함.
    let y = 10;
    println!("x = {} and y = {} and x+y = {}", x, y, x + y);
    
    /*************************************************************/
    
    // Shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);
    
    /*************************************************************/
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The spaces length is {spaces}.");

    /*************************************************************/
    
    let mut spaces = "   ";
    // spaces = spaces.len(); // Error!
}

// 상수는 불변성 그 자체이며 타입을 지정해야 함.
const MAX_POINTS: u32 = 100_000;