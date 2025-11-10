use rust_study_2025;

// tests 폴더는 특별하게 취급
// #[cfg(test)] mod tests { ... } 같은 형태의 테스트 모듈 필요 없음

mod common;

#[test]
fn test_org_it_adds_two() {
    assert_eq!(4, rust_study_2025::add_two(2));
    assert_eq!(7, common::test_org_setup());
    assert_eq!(1, 1);
}