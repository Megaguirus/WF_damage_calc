use crate::calculations::fill_sheet;

mod calculations;
mod mods;
mod weapons;

fn main() {
    for a in fill_sheet() {
        println!("{:?}", a);
    }
}
