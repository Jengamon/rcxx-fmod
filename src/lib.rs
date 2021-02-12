#[macro_use]
extern crate bitflags;

pub mod core;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp/core_system.hpp");
        type CoreSystem;

        fn Create_CoreSystem_Pointer() -> Result<UniquePtr<CoreSystem>>;
        fn init(&self, maxchannels: u32, initflags: u32) -> Result<()>; 
    }
}

#[allow(non_camel_case_types)]
pub mod flags {
    bitflags! {
        pub struct FMOD_INITFLAGS: u32 {
            const FMOD_INIT_NORMAL                 = 0x00000000;
            const FMOD_INIT_STREAM_FROM_UPDATE     = 0x00000001;
            const FMOD_INIT_MIX_FROM_UPDATE        = 0x00000002;
            const FMOD_INIT_3D_RIGHTHANDED         = 0x00000004;
            const FMOD_INIT_CHANNEL_LOWPASS        = 0x00000100;
            const FMOD_INIT_CHANNEL_DISTANCEFILTER = 0x00000200;
            const FMOD_INIT_PROFILE_ENABLE         = 0x00010000;
            const FMOD_INIT_VOL0_BECOMES_VIRTUAL   = 0x00020000;
            const FMOD_INIT_GEOMETRY_USECLOSEST    = 0x00040000;
            const FMOD_INIT_PREFER_DOLBY_DOWNMIX   = 0x00080000;
            const FMOD_INIT_THREAD_UNSAFE          = 0x00100000;
            const FMOD_INIT_PROFILE_METER_ALL      = 0x00200000;
            const FMOD_INIT_MEMORY_TRACKING        = 0x00400000;
        }
    }
}