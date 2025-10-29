// mod ch02_1_guessing_game;
// mod ch03_1_var_mutable;
// mod ch03_2_data_types;
// mod ch03_3_func;
// mod ch03_5_control_statement;
// mod ch04_1_ownership;
// mod ch04_2_references_borrowing;
// mod ch04_3_slices;
// mod ch05_1_Structure;
// mod ch05_2_Structure_Examples;
// mod ch05_3_Method_syntax;
// mod ch06_1_enum;
// mod ch06_2_Match_control_flow;
// mod ch06_3_if_let;
// pub mod ch07_garden;
// use crate::ch07_garden::ch07_vegetables::Asparagus;
// use std::collections::HashMap;
mod ch08_1_vector;

fn main() {
    // ch02_1_guessing_game::main();
    // ch03_1_var_mutable::main();
    // ch03_2_data_types::main();
    // ch03_3_func::main();
    // ch03_5_control_statement::main();
    // ch04_1_ownership::main();
    // ch04_2_references_borrowing::main();
    // ch04_3_slices::main();
    // ch05_1_Structure::main();
    // ch05_2_Structure_Examples::main();
    // ch05_3_Method_syntax::main();
    // ch06_1_enum::main();
    // ch06_2_Match_control_flow::main();
    // ch06_3_if_let::main();

    // 7.2 모듈을 정의하여 스코프 및 공개 여부 제어하기
    // https://doc.rust-kr.org/ch07-02-defining-modules-to-control-scope-and-privacy.html
    /*
    # crate[크레이트] 2가지 타입
    - binary crate: 실행가능한 실행 파일로 컴파일됨
    - library crate: main을 갖고 있지 않고 실행 파일 형태로 컴파일되지 않음.

    - 바이너리 크레이트는 src/main.rs가 크레이트 루트다. (관례)
    - cargo는 src/lib.rs 파일이 존재할 경우 해당 패키지명의 라이브러리 크레이트를 포함하고 있다고 판단함.
     */
    // let plant = Asparagus {};
    // println!("I'm growing {:?}!", plant);

    // 7.4 use 키워드로 경로를 스코프 안으로 가져오기
    // 보편적인 use 경로 작성법 - https://doc.rust-kr.org/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
    // 예제 7-14
    // let mut map = HashMap::new();
    // map.insert(1,2);

    ch08_1_vector::main();
}