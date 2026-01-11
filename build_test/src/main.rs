use crate::avro::Person;

mod avro {
    include!(concat!(env!("OUT_DIR"), "/person.rs"));
}

fn main() {
    let _person = Person::default();
    panic!("This crate was created just to test avro schema compiling in build scripts!");
}
