// 구조체 정의
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct NewUser {
    username: String,
    email: String,
    active: bool,
    favorite_color: ColorRGB,
    location: Point,
}

pub fn main () {
    // User 구조체로 instance 생성 (실체화 한 것)
    // 필드 순서는 상이해도 된다.
    let mut user1 = User {
        email: String::from("rokag3@gmail.com"),
        username: String::from("홍길동"),
        active: true,
        sign_in_count: 1
    };

    // 구조체 인스턴스의 값을 변경하거나 읽을 수 있다.
    user1.email = String::from("jwoo.kim@nextsecurities.com"); // 물론 user1 이 mutable인 경우에만 변경 가능하겠지.

    // 구조체 인스턴스의 값 읽기
    println!("user1 email = {}", user1.email);

    /******************************************************/

    // 구조체 업데이트 문법 (struct update syntax)

    // 신규 구조제 user2 을 만들 때 기존 구조체 user1의 일부 값을 그대로 가져올 수 있음
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // 더 적은 코드로 동일한 효과를 낼 수 있음.
    // email 필드는 새로운 값으로 set 하되, 나머지 모든 필드들은 user2의 필드 값을 그대로 사용하기
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2 // 이건 제일 끝에 적어줘야 한다.
    };

    /******************************************************/

    // 명명된 필드 없는 튜플 구조체를 사용하여 다른 타입 만들기

    let black = ColorRGB(0, 0, 0);
    let origin_pos = Point(26.0, -106.0);

    println!("Black RGB is {}, {}, {}", black.0, black.1, black.2);
    println!("origin_pos is {}, {}", origin_pos.0, origin_pos.1);

    let user5 = NewUser {
        username: String::from("Derek Sanderson Jeter"),
        email: String::from("rokag3@gmail.com"),
        active: true,
        favorite_color: ColorRGB(120, 64, 87),
        location: Point(26.1231525091274, 157.34144124124),
    };

    println!("user5.favorite_color is {}, {}, {}", user5.favorite_color.0, user5.favorite_color.1, user5.favorite_color.2);

    /******************************************************/
    
    // 필드가 없는 유사 유닛 구조체
    // unit-like structs
    
    let subject = AlwaysEqual;
}

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username, // 구조체 안에 필드명과 변수명이 동일하다면 생략 가능. -> 필드 초기화 축약법 (field init shorthand)
        active: true,
        sign_in_count: 1,
    }
}

// Tuple Struct (튜플 구조체)
// 구조체 자체에는 이름을 지어 의미를 부여하지만, 이를 구성하는 각 필드에는 이름을 붙이지 않고 타입만 지정.
struct ColorRGB(u16, u16, u16);
struct Point(f32, f32);