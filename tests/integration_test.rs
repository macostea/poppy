use poppy::engine::PoppyEngine;

#[test]
fn it_parses_script() {
    let mut engine = PoppyEngine::new();
    engine.load_script("tests/data/simple_script.rhai".into()).unwrap();
    engine.run_script().unwrap();
}