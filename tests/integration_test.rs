use poppy::engine::PoppyEngine;

#[test]
fn it_parses_script() {
    let mut engine = PoppyEngine::new();
    engine.load_script("tests/data/simple_script.rhai".into()).unwrap();
    let ok = engine.run_script();

    assert!(ok.is_ok());
}

#[test]
fn sh_no_node() {
    let mut engine = PoppyEngine::new();
    engine.load_script("tests/data/sh_no_node.rhai".into()).unwrap();
    let err = engine.run_script();

    assert!(err.is_err());
}

#[test]
fn sh_node() {
    let mut engine = PoppyEngine::new();
    engine.load_script("tests/data/sh_with_node.rhai".into()).unwrap();
    let ok = engine.run_script();

    assert!(ok.is_ok());
}

#[test]
fn sh_file_node() {
    let mut engine = PoppyEngine::new();
    engine.load_script("tests/data/sh_file.rhai".into()).unwrap();
    let ok = engine.run_script();

    assert!(ok.is_ok());
}