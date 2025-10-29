pub mod hosting;

pub mod front_of_house {
    // 모듈을 공개했다고 해서 내용까지 공개되는 것은 아니다.
    // 모듈은 단순한 컨테이너이기 때문에, 모듈을 공개하는 것 만으로 할 수 있는 것은 별로 없음.
    // 여기에 더해, 모듈이 가지고 있는 아이템도 마찬가지로 공개해야 함.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_over() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}