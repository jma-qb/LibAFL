use std::fmt::Debug;

use libafl::Error;

// use crate::modules::{EmulatorModule, EmulatorModuleTuple};
use crate::modules::EmulatorModule;

/// A conditionally enabled module.
/// If the closure returns true, the wrapped module will be used, else it will be skipped.
#[derive(Debug)]
pub struct IfModule<CB, MD> {
    closure: CB,
    if_module: MD,
}

impl<CB, MD> IfModule<CB, MD> {
    /// Constructor for this conditionally enabled module.
    /// If the closure returns true, the wrapped module will be used, else it will be skipped.
    #[allow(dead_code)]
    pub fn new(closure: CB, module: MD) -> Self {
        Self {
            closure,
            if_module: module,
        }
    }
}

impl<CB, MD, I, S> EmulatorModule<I, S> for IfModule<CB, MD>
where
    I: Unpin,
    S: Unpin,
    CB: FnMut() -> Result<bool, Error> + 'static,
    MD: super::EmulatorModuleTuple<I, S> + 'static,
{
    const HOOKS_DO_SIDE_EFFECTS: bool = true;

    fn pre_qemu_init<ET>(
        &mut self,
        emulator_modules: &mut crate::EmulatorModules<ET, I, S>,
        qemu_params: &mut crate::QemuParams,
    ) where
        ET: super::EmulatorModuleTuple<I, S>,
    {
        if (self.closure)().unwrap_or(false) {
            self.if_module
                .pre_qemu_init_all(emulator_modules, qemu_params);
        }
    }

    fn post_qemu_init<ET>(
        &mut self,
        qemu: crate::Qemu,
        emulator_modules: &mut crate::EmulatorModules<ET, I, S>,
    ) where
        ET: super::EmulatorModuleTuple<I, S>,
    {
        if (self.closure)().unwrap_or(false) {
            self.if_module.post_qemu_init_all(qemu, emulator_modules);
        }
    }

    fn first_exec<ET>(
        &mut self,
        qemu: crate::Qemu,
        emulator_modules: &mut crate::EmulatorModules<ET, I, S>,
        state: &mut S,
    ) where
        ET: super::EmulatorModuleTuple<I, S>,
    {
        if (self.closure)().unwrap_or(false) {
            self.if_module.first_exec_all(qemu, emulator_modules, state);
        }
    }

    fn pre_exec<ET>(
        &mut self,
        qemu: crate::Qemu,
        emulator_modules: &mut crate::EmulatorModules<ET, I, S>,
        state: &mut S,
        input: &I,
    ) where
        ET: super::EmulatorModuleTuple<I, S>,
    {
        if (self.closure)().unwrap_or(false) {
            self.if_module
                .pre_exec_all(qemu, emulator_modules, state, input);
        }
    }

    fn post_exec<OT, ET>(
        &mut self,
        qemu: crate::Qemu,
        emulator_modules: &mut crate::EmulatorModules<ET, I, S>,
        state: &mut S,
        input: &I,
        observers: &mut OT,
        exit_kind: &mut libafl::executors::ExitKind,
    ) where
        OT: libafl::observers::ObserversTuple<I, S>,
        ET: super::EmulatorModuleTuple<I, S>,
    {
        if (self.closure)().unwrap_or(false) {
            self.if_module.post_exec_all(
                qemu,
                emulator_modules,
                state,
                input,
                observers,
                exit_kind,
            );
        }
    }

    unsafe fn on_crash(&mut self) {
        unsafe {
            self.if_module.on_crash_all();
        }
    }

    unsafe fn on_timeout(&mut self) {
        unsafe {
            self.if_module.on_timeout_all();
        }
    }
}
