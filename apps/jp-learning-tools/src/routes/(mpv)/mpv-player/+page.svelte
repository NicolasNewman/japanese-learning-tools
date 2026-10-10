<script lang="ts">
    import { mpvState } from "../../../stores/mpvState.svelte";
    import {
        type MpvObservableProperty,
        type MpvConfig,
        init,
        observeProperties,
        command,
        destroy,
        listenEvents,
    } from "tauri-plugin-libmpv-api";
    import { resourceDir, sep } from "@tauri-apps/api/path";
    import {
        addToWatchHistory,
        toMpvScriptOpt,
        get,
    } from "$lib/persistence/mpvStore";
    const OBSERVED_PROPERTIES = [
        ["pause", "flag"],
        ["time-pos", "double", "none"],
        ["duration", "double", "none"],
        ["filename", "string", "none"],
        ["sub-delay", "double", "none"],
    ] as const satisfies MpvObservableProperty[];
    let subOffset: number = 0;
    let timestamp: number = 0;

    const _startMPV = async () => {
        console.log("Starting MPV...");
        try {
            const resourcePath = await resourceDir();
            const mpvPath = `${resourcePath}${sep()}resources${sep()}mpv${sep()}`;
            const mpvConfig: MpvConfig = {
                initialOptions: {
                    alang: "ja,jp,jpn,japanese,en,eng,english,English,enUS,en-US",
                    slang: "ja,jp,jpn,japanese,en,eng,english,English,enUS,en-US",
                    // TODO: make these configurable
                    "screenshot-directory": "~/Pictures/Screenshots/",
                    "screenshot-template": "%F_%wHh%wMm%wSs%wTms",
                    "sub-auto": "fuzzy",
                    "subs-with-matching-audio": "yes",
                    "screenshot-format": "jpg",
                    "screenshot-jpeg-quality": 90,
                    "screenshot-high-bit-depth": "yes",
                    "sub-font-size": 40,
                    vo: "gpu-next",
                    hwdec: "auto-safe",
                    "keep-open": "yes",
                    "force-window": "yes",
                    "load-scripts": "no",
                    "script-opts": toMpvScriptOpt(
                        (await get("script-opts")) ?? [],
                    ),
                    // "input-conf": `${mpvPath}input.conf`,
                    // "script-opts":
                    // "subs2srs-autoclip_method=clipboard,subs2srs-autoclip=yes,subs2srs-deck_name=Active::Japanese::subs2srs,subs2srs-note_tag=subs2srs kanji-first",
                    include: `${mpvPath}input.conf`,
                    "osd-fonts-dir": `${mpvPath}fonts${sep()}`,
                    "osd-font": "Material Design Iconic Font",
                    osc: "no",
                },
                observedProperties: OBSERVED_PROPERTIES,
            };
            await init(mpvConfig);
            await command("load-script", [
                `${mpvPath}scripts${sep()}mpvacious${sep()}`,
            ]);
            await command("load-config-file", [
                `${mpvPath}script-opts${sep()}subs2srs.conf`,
            ]);

            await command("load-script", [
                `${mpvPath}scripts${sep()}ModernZ${sep()}`,
            ]);

            await command("load-input-conf", [`${mpvPath}input.conf`]);

            if (mpvState.mediaFile) {
                await command("loadfile", [mpvState.mediaFile]);
                // watch history id may have been set if the user clicked on a history item
                if (mpvState.watchHistoryId === null) {
                    addToWatchHistory(mpvState.mediaFile ?? "").then((id) => {
                        mpvState.watchHistoryId = id;
                    });
                }
            }
            mpvState.unlisten = await observeProperties(
                OBSERVED_PROPERTIES,
                async ({ name, data }) => {
                    switch (name) {
                        case "pause":
                            console.log("Playback paused state:", data);
                            break;
                        case "time-pos":
                            if (data) {
                                timestamp = data;
                            }
                            break;
                        case "duration":
                            console.log("Duration:", data);
                            break;
                        case "sub-delay":
                            console.log("Subtitle delay:", data);
                            if (data) {
                                subOffset = data;
                            }
                            break;
                        case "filename":
                            console.log("Current playing file:", data);
                            break;
                    }
                },
            );

            mpvState.unlistenEvents = await listenEvents(async (e) => {
                switch (e.event) {
                    case "file-loaded":
                        console.log("File loaded:");
                        if (mpvState.restorePoint) {
                            await command("seek", [
                                Math.round(mpvState.restorePoint.timestamp),
                                "absolute+keyframes",
                            ]);
                            await command("set", [
                                "sub-delay",
                                mpvState.restorePoint.subOffset,
                            ]);
                            mpvState.restorePoint = null;
                        }

                        break;
                }
            });

            mpvState.isRunning = true;
            mpvState.isLoading = false;
        } catch (error) {
            console.error("Failed to start MPV:", error);
        }
    };
    // _startMPV();
</script>

<div></div>
