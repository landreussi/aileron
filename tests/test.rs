#[test]
fn default_macro_call() {
    aileron::include_avro!("tests/person.avsc");
}

#[test]
fn custom_macro_call() {
    aileron::include_avro!(
        "tests/person.avsc",
        precision = 4,
        impl_avro_schema = Derive,
        derive_builders = true,
        use_chrono_dates = true,
        use_avro_rs_unions = true,
        extra_derives = ["rkyv::Archive", "rkyv::Serialize", "Default"],
    );

    // default is implemented!
    Person::default();

    // builder now exists!
    PersonBuilder::create_empty();
}
