#include "null_engine/NullEngine.h"
#include "engine/EngineUtil.h"
#include "engine/Paths.h"
#include <vector>
#include <string_view>

namespace {
    IEngineClient* g_client = nullptr;
}

void NullEngine::RegisterClient(IEngineClient& client) {
    g_client = &client;
}

bool NullEngine::Run(int argc, char** argv) {
    if (!EngineUtil::FindAndSetRootPath(fs::path(fs::absolute(argv[0]))))
        return false;

    std::shared_ptr<IEngineService> engineService =
        std::make_shared<aggregate_adapter<IEngineService>>(
            // SetFocusMainWindow
            [] {},
            // SetFocusConsole
            [] {},
            // ResetOverlay
            [](const char* /*file*/) {});

    // Build argument list expected by IEngineClient::Init
    std::vector<std::string_view> args;
    args.reserve(static_cast<size_t>(argc));
    for (int i = 0; i < argc; ++i) {
        args.emplace_back(argv[i]);
    }

    // Keep a stable string backing for bios rom path while calling Init
    const std::string biosRomPath = Paths::biosRomFile.string();
    if (!g_client->Init(args, engineService, std::string_view{biosRomPath})) {
        return false;
    }

    bool quit = false;
    while (!quit) {
        double frameTime = 1.0 / 60;
        EmuEvents emuEvents{};
        Options options{};
        Input input{};
        RenderContext renderContext{};
        AudioContext audioContext{0};

        if (!g_client->FrameUpdate(frameTime, {std::ref(emuEvents), std::ref(options)}, input,
                                   renderContext, audioContext)) {
            quit = true;
        }
    }

    return true;
}
