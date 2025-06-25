import type { KanjiBankData, Level } from "./kanji-bank";
import type { Component } from 'svelte';
import WaniKaniSettings from './wanikani/settings.svelte';
const WaniKaniImporter = import("./wanikani/index");

abstract class Importer{
  abstract load(): Promise<KanjiBankData>;
  abstract serviceLevelToStage(level: number): Level;
}

const importerSettingsPage: Record<KanjiSource, Component> = {
  'wanikani': WaniKaniSettings
}

const kanjiImporter: Record<KanjiSource, (...params: any) => Promise<Importer>> = {
  'wanikani': async (apiKey: string) => new ((await WaniKaniImporter).default)(apiKey)
}

export { Importer }
export type KanjiSource = 'wanikani';
export {kanjiImporter, importerSettingsPage};