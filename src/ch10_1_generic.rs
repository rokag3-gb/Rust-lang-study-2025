pub fn main() {
    // 함수로 추출하여 코드에 중복 없애기

    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    /**********************************************/

    // 이 동일한 로직이 하나 더 생기면??
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    /**********************************************/

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("코드 중복을 줄여서 이제 행복해졌다. The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("코드 중복을 줄여서 이제 행복해졌다. The largest number is {}", result);

    /**********************************************/

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list); // 각각 다른 함수
    println!("i32에 대한 가장 큰 값은 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list); // 각각 다른 함수
    println!("char에 대한 가장 큰 값은 {}", result);

    /**********************************************/

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_generic(&number_list); // 하나의 함수로 처리 가능
    println!("[Generic 적용된 상태] 가장 큰 값은 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generic(&char_list); // 하나의 함수로 처리 가능
    println!("[Generic 적용된 상태] 가장 큰 값은 {}", result);

    /**********************************************/

    // 제네릭 구조체 정의

    let integer = Point { x: 5, y: 10 }; // 각 요소 값은 동일 타입이어야 함.
    let float = Point { x: 1.0, y: 4.0 };

    // 다른 타입이면?
    // error[E0308]: mismatched type
    // expected integer, found floating-point number

    // struct Point2 로 해결
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    /**********************************************/

    // 제네릭 열거형 정의

    // enum Option<T>
    // enum Result<T, E>

    /**********************************************/

    // 제네릭 메서드 정의

    let p = Point { x: 7, y: 10 };

    println!("p.method_x() = {}", p.method_x());

    let pf = Point { x: 7.2, y: 10.4 };
    println!("pf.distance_from_origin() = {}", pf.distance_from_origin());

    /**********************************************/

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c

    /**********************************************/

    // 트레이트로 공통된 동작을 정의하기

}

/**********************************************/

// pub trait Summary {
//     fn summarize_author(&self) -> String;
//     // 구현체에서 summarize() 가 이런 모습으로 구현되어 있어야 해!
//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }
//
// pub struct NewsArticle {
//     pub headline: String,
//     pub content: String,
// }
//
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} - {}", self.headline, self.content)
//     }
// }

/**********************************************/
// Point3 정의

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

/**********************************************/

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> { // 제네릭 메서드 정의
    fn method_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> { // 이와 동시에 non-generic method도 정의
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // (x² + y²)의 제곱근 => 원정(0, 0) ~ Point<f32> 까지의 거리
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

/**********************************************/

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 에러!
// 지금 10장에서는 이 에러가 ‘largest의 본문이 T가 될 수 있는 모든 타입에 대해 동작할 수 없음’을 뜻한다는 정도만 알아두자
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 사실 이렇게 하면 됨
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}