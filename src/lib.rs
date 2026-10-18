//! `kv_map` — pure-`.hnl` functional key-value map (store-as-value).
#![allow(dead_code)]

pub const KV_MAP_HNL: &str = include_str!("index.hnl");
pub const KV_MAP_FUNCS: &[&str] = &["new", "set", "get", "has", "remove", "clear", "size", "keys"];
