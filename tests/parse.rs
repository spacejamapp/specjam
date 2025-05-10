//! Parse all tests

use specjam::{Registry, Trace};
use std::path::PathBuf;

#[test]
fn parse_all() {
    let registry = Registry::new(PathBuf::from("jamtestvectors"));

    registry.accumulate().unwrap();
    registry.assurances().unwrap();
    registry.authorizations().unwrap();
    registry.codec().unwrap();
    registry.disputes().unwrap();
    registry.history().unwrap();
    registry.preimages().unwrap();
    registry.pvm().unwrap();
    registry.reports().unwrap();
    registry.safrole().unwrap();
    registry.statistics().unwrap();
    registry.shuffle().unwrap();
    registry.trace(Trace::Fallback).unwrap();
    registry.trace(Trace::Safrole).unwrap();
    registry.trace(Trace::ReportsL0).unwrap();
    registry.trie().unwrap();
}
