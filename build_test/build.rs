fn main() {
    avrogant::AvroCompiler::new()
        .extra_derives(vec!["Default".to_string()])
        .compile(&["../avrogant/tests/person.avsc"])
        .unwrap();
}
