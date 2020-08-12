#![feature(proc_macro_hygiene, decl_macro)]

extern crate recipes_tool;

use recipes_tool::init_application;

fn main() {
        init_application().launch();
}
