extern crate reqwest;
extern crate strum;
#[macro_use] extern crate strum_macros;

mod bizhawk;

fn main() {
    let pkm_red = bizhawk::Bizhawk::new();
    // some test fun, write FF to all WRAM
    println!("{:?}", pkm_red.write_slice(bizhawk::MemRegion::WRAM, 0x0000, vec!(0xFF;0x2000)).unwrap());
}
