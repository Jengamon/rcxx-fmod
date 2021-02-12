#include <fmod.hpp>
#include <fmod_errors.h>
#include <memory>
#include <stdexcept>

struct CoreSystem {
    CoreSystem() {
        this->sys = NULL;
    }

    void init(size_t maxchannels, uint32_t flags) const;

    FMOD::System* sys;
};

// The Core System binding

std::unique_ptr<CoreSystem> Create_CoreSystem_Pointer() {
    std::unique_ptr<CoreSystem> coreSystemPtr(new CoreSystem);

    auto ret = FMOD::System_Create(&coreSystemPtr->sys);
    if(ret != FMOD_OK) {
        throw new std::runtime_error(FMOD_ErrorString(ret));
    }

    return coreSystemPtr;
}

void CoreSystem::init(size_t maxchannels, uint32_t flags) const {
    auto ret = this->sys->init(maxchannels, flags, NULL);
    if(ret != FMOD_OK) {
        throw new std::runtime_error(FMOD_ErrorString(ret));
    }
}