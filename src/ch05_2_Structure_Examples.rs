pub fn main() {
    println!("5.2 구조체를 사용한 예제 프로그램");

    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    /***************************************************/

    // 튜플로 리팩토링하기
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area_with_tuple(rect1));

    /***************************************************/

    // 구조체로 리팩터링하여 코드에 더 많은 의미를 담기
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.",
             area_with_struct(&rect1)
    );

    /***************************************************/

    // 트레이트(trait) 파생으로 유용한 기능 추가하기
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {}", rect1);
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

    println!("rect1 is {:?}", rect1);
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    // help: the trait `Debug` is not implemented for `Rectangle`
    // note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

    // println!("rect1 is {:?}", rect1);
    // "Rectangle { width: 30, height: 50 }"

    println!("rect1 is {:#?}", rect1);
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    /***************************************************/

    // dbg! 매크로
    // 코드가 어떤 일을 하고 있는지 알아볼 때 매우 유용할 수 있다.
    
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // [src\ch05_2_Structure_Examples.rs:59:16] 30 * scale = 60
        height: 50,
    };

    dbg!(&rect1);
    // [src\ch05_2_Structure_Examples.rs:63:5] &rect1 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)] // 외부 속성
struct Rectangle {
    width: u32,
    height: u32
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}