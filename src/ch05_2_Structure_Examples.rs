pub fn main() {
    println!("5.2 구조체를 사용한 예제 프로그램");
    
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}