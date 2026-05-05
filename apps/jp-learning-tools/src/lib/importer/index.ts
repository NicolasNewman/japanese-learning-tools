import type { Component } from "svelte";
import WaniKaniSettings from "./wanikani/settings.svelte";
import AnkiSettings from "./anki/settings.svelte";
import type {
  AnkiMetadata,
  KanjiBankData,
  KanjiSource,
  Level,
  WaniKaniMetadata,
} from "@nicolasnewman/kanji-bank-types";
import type { AnkiSettingsStore } from "./anki/store";

export type SaveState = "IDLE" | "SAVING" | "SAVED";

const WaniKaniImporter = import("./wanikani/index");
const AnkiImporter = import("./anki/index");

abstract class Importer<T> {
  // TODO: full implementation of version
  abstract version: number;
  abstract load(): Promise<KanjiBankData<T>>;
  abstract serviceLevelToStage(level: number | AnkiMetadata["stats"]): Level;
}

const importerSettingsPage: Record<KanjiSource, Component> = {
  wanikani: WaniKaniSettings,
  anki: AnkiSettings,
};

const kanjiImporter: Record<
  KanjiSource,
  (...params: any) => Promise<Importer<SourceMetadata>>
> = {
  wanikani: async (apiKey: string) =>
    new (await WaniKaniImporter).default(apiKey),
  anki: async (
    models: NonNullable<AnkiSettingsStore["syncedModels"]>,
    deckNames: string[],
  ) => new (await AnkiImporter).default(models, deckNames),
};

export { Importer };
export type SourceMetadata = WaniKaniMetadata | AnkiMetadata;
export { kanjiImporter, importerSettingsPage };
