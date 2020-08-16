#![feature(proc_macro_hygiene, decl_macro)]

extern crate recipes_tool;

use recipes_tool::init_application;
use recipes_tool::init_logging;

fn main() {
        init_logging();
        init_application().launch();
}
