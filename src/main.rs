#[macro_use]
mod macros;
mod formatting;
mod arr_and_slices;
mod enums;
mod structures;

fn main() {
    formatting::test();
    arr_and_slices::test();
    enums::test();
    structures::test();
}