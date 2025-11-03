use std::fmt::{self, Display};

pub fn main() {

    // 트레이트는 타입에 "행동 계약"을 부여함. 실제 데이터는 구조체(NewsArticle, Tweet)로 보유.
    // 각 타입은 Summary 트레이트를 impl 하여 summarize 동작을 제공(오버라이드 가능).
    let article = NewsArticle {
        headline: "Rust 1.80 Released".into(), // into() : Into 트레잇 에 정의되어 있는 타입 변환 메서드. convert랑 비슷.
        location: "Seoul".into(),
        author: "Alice".into(),
        content: "Performance and ergonomics improvements...".into(),
    };

    let tweet = Tweet {
        username: "bob".into(),
        content: "Generics + Traits + Lifetimes = 사랑️".into(),
        reply: false,
        retweet: true,
    };

    // 트레이트 메서드 호출
    // 디스패치는 정적(모노모픽)으로 일어남: 제네릭 + 트레이트 바운드는 컴파일 시 구체 타입별 코드를 정적으로 다 생성해버림

    // 기본 메서드(default method)와 오버라이드 예시 비교.
    println!("article.summarize() => {}", article.summarize()); // 오버라이드 버전
    println!("tweet.summarize()   => {}", tweet.summarize());   // 기본 구현 사용
    println!();

    // 파라미터 바운드 (impl Trait / 제네릭 + 바운드)
    // impl Trait: 간단 문법으로 “이 트레이트를 구현한 어떤 타입이든” 받기
    // 제네릭 바운드: 동일 의미지만 여러 트레이트를 조합하거나 재사용성 증가
    notify(&article);
    notify(&tweet);
    notify_pretty(&tweet); // Display + Summary 동시 요구
    println!();

    // where 절 예시: 복잡한 바운드를 가독성 있게 분리하여 선언. 읽기 쉬운 시그니처
    notify_pair(&tweet, &article);
    println!();

    // impl Trait 반환 사용
    // “구체 타입을 숨기고 트레이트만 노출”하는 패턴(opaque return type)
    // 호출자는 Summary만 알면 되고, 내부 구현 교체 용이(캡슐화)
    let s = returns_summarizable();
    println!("returns_summarizable() => {}", s.summarize());
    println!();

    // 6) 블랭킷(Blanket) 구현
    // 특정 트레이트를 만족하는 모든 타입에 일괄로 새 기능 부여 (표준의 ToString for T: Display와 유사).
    // 제약 기반 확장으로 중복 제거 및 일관성 확보.
    article.quick_alert();
    tweet.quick_alert();

    // 7) 제약 기반 메서드(Pair 예시)
    // Display + PartialOrd 제약으로 "비교 가능하고 출력 가능한" T에만 메서드 제공.
    // 제약이 곧 문서: 가능한 연산을 타입 시스템으로 명시.
    let p = Pair::new(10, 42);
    p.cmp_display();
}

/// 트레이트: 공통 동작(메서드) 규약 정의
/// summarize_author 는 구현체가 반드시 구현해야 한다 (필수 메서드)
/// summarize는 기본 구현(default impl) 제공 -> 필요 시 구현체에서 오버라이드 가능함
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String { // override
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle { // NewsArticle에 대한 Summary 구현
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String { // 기본 구현 대신 나만의 summarize를 제공 (override)
        format!(
            "{} by {} ({})",
            self.headline, self.author, self.location
        )
    }
}

impl Summary for Tweet { // Tweet에 대한 Summary 구현
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize는 기본 구현 사용
}

// Display 구현: 예시로 Tweet/NewsArticle을 사람이 읽을 수 있게 출력 형태 제공
impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@{}: {}{}{}",
            self.username,
            self.content,
            if self.reply { " (reply)" } else { "" },
            if self.retweet { " (retweet)" } else { "" },
        )
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Article] {} — {} @ {}",
            self.headline, self.author, self.location
        )
    }
}

// impl Trait
// 참조 구현체 를 인수로 받기 (&impl)
// Summary를 구현한 아무 타입이나 받기
fn notify(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}

// 제네릭 + 트레이트 바운드
// 동시 제한도 가능 -> T: Summary + Display
fn notify_pretty<T: Summary + Display>(item: &T) {
    println!("notify_pretty: {}", item);         // Display 사용
    println!(" summary: {}", item.summarize());   // Summary 사용
}

// where 절로 가독성 개선
fn notify_pair<T, U>(a: &T, b: &U)
where
    T: Summary + Display,
    U: Summary,
{
    println!("pair[0] = {}", a); // Display
    println!("pair[0] summary = {}", a.summarize());
    println!("pair[1] summary = {}", b.summarize());
}

// impl Trait 반환: 호출자는 Summary를 반환받지만 구체 타입은 숨김
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rustacean"),
        content: String::from("Traits are powerful!"),
        reply: false,
        retweet: false,
    }
}

// 블랭킷(Blanket) 스타일 패턴 예시:
// Summary를 구현한 모든 타입에 대해 QuickAlert 트레이트를 자동 구현
// (표준 라이브러리의 ToString이 Display에 대해 blanket 구현되는 것과 유사한 아이디어)
pub trait QuickAlert {
    fn quick_alert(&self);
}

impl<T> QuickAlert for T
where
    T: Summary,
{
    fn quick_alert(&self) {
        println!("QUICK ALERT !!! {}", self.summarize());
    }
}

// Display + PartialOrd 제약을 이용한 메소드 예시
// 두 값 중 큰 값을 예쁘게 표시
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("max is x = {}", self.x);
        } else {
            println!("max is y = {}", self.y);
        }
    }
}