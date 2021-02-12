#ifndef RCXXFMOD_CORE_SYSTEM_HPP
#define RCXXFMOD_CORE_SYSTEM_HPP

#include <fmod.hpp>
#include <fmod_errors.h>
#include <memory>

struct CoreSystem {
    CoreSystem() {
        this->sys = NULL;
    }

    void init(uint32_t maxchannels, uint32_t flags) const;

    FMOD::System* sys;
};

std::unique_ptr<CoreSystem> Create_CoreSystem_Pointer();

#endif // RCXXFMOD_CORE_SYSTEM_HPP