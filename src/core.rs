use super::ffi;

pub struct System {
    sys: cxx::UniquePtr<ffi::CoreSystem>,
}

impl System {
    pub fn new() -> Result<System, String> {
        match ffi::Create_CoreSystem_Pointer() {
            Ok(sys) => Ok(System {
                sys
            }),
            Err(e) => Err(format!("Couldn't initialize FMOD System: {}", e))
        }
    }
}