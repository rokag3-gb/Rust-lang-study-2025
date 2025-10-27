pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("사각형 면적은 {} 이다."
             , rect1.area() // 메소드 문법
    );

    // width 뒤에 () 을 붙이면 메소드, 안붙이면 필드로 인식함.
    println!("너비가 0보다 큰가? {}"
             , rect1.width()
    );

    /****************************************************/

    // 더 많은 매개변수를 가진 메서드

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2 ? -> {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? -> {}", rect1.can_hold(&rect3));

    println!("Can rect3 hold rect2 ? -> {}", rect3.can_hold(&rect2));

    /****************************************************/

    // 연관 함수 (Associated Function)

    let rect4 = Rectangle::square(20);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 이 impl 블럭이 struct Rectangle 의 구현체이다.
// struct와 impl의 이름이 Rectangle으로 동일하면 "아, 이건 struct Rectangle의 구현체구나" 라고 자동으로 인식함.
impl Rectangle { // implementation (구현)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 여러 개의 implementation block
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool { // 내가 (self) other 보다 가로, 세로가 모두 길면 true, 아니면 false
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}