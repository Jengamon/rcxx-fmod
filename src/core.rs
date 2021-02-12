use super::{ffi, flags};

pub struct System {
    sys: cxx::UniquePtr<ffi::CoreSystem>,
}

impl System {
    pub fn new(maxchannels: usize, flags: flags::FMOD_INITFLAGS) -> Result<System, String> {
        match ffi::Create_CoreSystem_Pointer() {
            Ok(sys) => match sys.init(maxchannels, flags.bits()) {
                Ok(()) => Ok(System {
                    sys
                }),
                Err(e) => Err(format!("Couldn't initialize FMOD System: {}", e))
            },
            Err(e) => Err(format!("Couldn't initialize FMOD System: {}", e))
        }
    }
}