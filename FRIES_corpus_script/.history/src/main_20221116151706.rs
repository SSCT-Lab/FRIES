use command::run_command;

mod command;
mod project;

fn main() {
    let f = |x| x + 1;
    run_command();
}
