#![allow(dead_code, unused)]

pub trait PrintName {
    fn name() -> &'static str;
    fn print_name() {
        println!("{}", Self::name());
    }
}
