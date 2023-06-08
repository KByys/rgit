use std::env;
use rgit::run;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    run(args);
}
