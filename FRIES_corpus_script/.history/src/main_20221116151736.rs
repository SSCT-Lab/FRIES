use command::run_command;

mod command;
mod project;

fn main() {
    let f = |x| x + 1.0;
    let x = f(10);
    run_command();
}
