use crate::*;

#[test]
fn stack_vm() -> Result<(), String> {
    use stackvm::*;
    let consts = vec![
        5., 2.
    ];
    let program = vec![
        StackInstr::new(StackByteCode::Const, 0),
        StackInstr::new(StackByteCode::Set, 0),
        StackInstr::new(StackByteCode::Const, 1),
        StackInstr::new(StackByteCode::Set, 1),
        StackInstr::new(StackByteCode::Get, 0),
        StackInstr::new(StackByteCode::Get, 1),
        StackInstr::new(StackByteCode::Add, 0),
        StackInstr::new(StackByteCode::Halt, 0),
    ];
    let mut vm = StackVM::new(program, consts, 2);
    vm.run()?;
    assert_eq!(vm.pop(), 7.);
    Ok(())
}