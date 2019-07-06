#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod error;
mod ignite;
mod pages;
mod watcher;

use config::Config;
use crossbeam_utils::sync::ShardedLock;
use std::any::Any;
use std::fmt::Display;
use std::sync::Arc;

type SharedConfig = Arc<ShardedLock<Config>>;

fn main() {
    let cfg = necessary(config::load(config::path()));
    necessary(config::verify(&cfg));

    let cfg = Arc::new(ShardedLock::new(cfg));
    let _watcher = necessary(watcher::watch(cfg.clone()));

    necessary(ignite::start(cfg));
}

fn necessary<T: Any, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Fatal Error: {}", error);
            std::process::exit(1);
        }
    }
}
