extern crate rcxx_fmod;

fn main() {
    // Temp
    let system = rcxx_fmod::core::System::new(2, rcxx_fmod::flags::FMOD_INITFLAGS::FMOD_INIT_NORMAL).unwrap();
}