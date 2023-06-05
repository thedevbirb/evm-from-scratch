use evm_from_scratch_new::evm::{
    evm::EVM,
    utils::types::{AccruedSubstate, GlobalState, Input, MachineState},
};

fn main() {
    let mut global_state = GlobalState::new();
    let mut accrued_substate = AccruedSubstate::new();
    let mut machine_state = MachineState::new();
    let mut input = Input::new_demo();

    EVM::execute(global_state, machine_state, accrued_substate, input);
}
