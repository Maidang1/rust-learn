
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("front_of_house");
        }

        pub fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("take_order");
        }

        pub fn server_order() {
            println!("server_order");
        }

        pub fn take_payment() {
            println!("take_payment");
        }
    }
}
