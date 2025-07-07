import type { KanjiBankData, Level } from "./kanji-bank";
import type { Component } from 'svelte';
import WaniKaniSettings from './wanikani/settings.svelte';

import type { WaniKaniMetadata } from './wanikani/index';
const WaniKaniImporter = import("./wanikani/index");


abstract class Importer<T> {
  abstract load(): Promise<KanjiBankData<T>>;
  abstract serviceLevelToStage(level: number): Level;
}

const importerSettingsPage: Record<KanjiSource, Component> = {
  'wanikani': WaniKaniSettings
}

const kanjiImporter: Record<KanjiSource, (...params: any) => Promise<Importer<SourceMetadata>>> = {
  'wanikani': async (apiKey: string) => new ((await WaniKaniImporter).default)(apiKey)
}

export { Importer }
export type KanjiSource = 'wanikani';
export type SourceMetadata = WaniKaniMetadata;
export { kanjiImporter, importerSettingsPage };