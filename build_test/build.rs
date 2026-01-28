fn main() {
    avrogant::AvroCompiler::new()
        .extra_derives(["Default"])
        .compile(&["../avrogant/tests/person.avsc"])
        .unwrap();
}
