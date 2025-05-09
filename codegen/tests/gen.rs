use specjam_codegen as codegen;
use std::path::Path;

#[test]
fn test_codegen() {
    let vectors = Path::new("../jamtestvectors");
    let output = Path::new("/tmp");

    let registry = codegen::Registry::new(vectors, output);
    registry.run().unwrap();
}
