use advtools::input::iter_input;
use advent16::Machine;

fn main() {
    let mut machine = Machine::new(iter_input());
    machine.run();
    advtools::verify("Register a for c = 0", machine.get_reg(0), 318003);

    machine.reset();
    machine.set_reg(2, 1);
    machine.run();
    advtools::verify("Register a for c = 1", machine.get_reg(0), 9227657);
}
