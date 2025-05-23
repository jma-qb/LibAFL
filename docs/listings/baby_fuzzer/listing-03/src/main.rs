extern crate libafl;
extern crate libafl_bolts;
use libafl::{
    executors::ExitKind,
    inputs::{BytesInput, HasTargetBytes},
};
use libafl_bolts::AsSlice;

fn main() {
    let mut harness = |input: &BytesInput| {
        let target = input.target_bytes();
        let buf = target.as_slice();
        if buf.len() > 0 && buf[0] == b'a' {
            if buf.len() > 1 && buf[1] == b'b' {
                if buf.len() > 2 && buf[2] == b'c' {
                    panic!("=)");
                }
            }
        }
        ExitKind::Ok
    };
    // To test the panic:
    let input = BytesInput::new(Vec::from("abc"));
    #[cfg(feature = "test-panic")]
    harness(&input);
}
