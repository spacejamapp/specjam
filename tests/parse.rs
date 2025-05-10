//! Parse all tests

use specjam::{Registry, Scale, Trace};
use std::path::PathBuf;

#[test]
fn parse_all() {
    let registry = Registry::new(PathBuf::from("jamtestvectors"));

    registry.accumulate(Scale::Tiny).unwrap();
    registry.assurances(Scale::Tiny).unwrap();
    registry.authorizations(Scale::Tiny).unwrap();
    registry.codec().unwrap();
    registry.disputes(Scale::Tiny).unwrap();
    registry.history().unwrap();
    registry.preimages().unwrap();
    registry.pvm().unwrap();
    registry.reports(Scale::Tiny).unwrap();
    registry.safrole(Scale::Tiny).unwrap();
    registry.statistics(Scale::Tiny).unwrap();
    registry.shuffle().unwrap();
    registry.trace(Trace::Fallback).unwrap();
    registry.trace(Trace::Safrole).unwrap();
    registry.trace(Trace::ReportsL0).unwrap();
    registry.trie().unwrap();
}
