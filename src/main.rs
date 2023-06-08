use iofs::env::current_program_path;
use iofs::fs::pbuilder::PathBuilder;
use std::env;
use rgit::run;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    // println!("{:?}", args)
    run(args);
}
