import type { KanjiBankData } from "./kanji-bank";
import type { Component } from 'svelte';
import WaniKaniSettings from './wanikani/settings.svelte';

abstract class Importer{
  abstract load(): Promise<KanjiBankData>;
}

const importKanji = async (): Promise<KanjiBankData> => {
    const WaniKaniImporter = (await import("./wanikani/index")).default;

    return {
        ...(await new WaniKaniImporter().load()),
    }
}

const importerSettingsPage: Record<KanjiSource, Component> = {
  'wanikani': WaniKaniSettings
}

export { Importer }
export type KanjiSource = 'wanikani';
export {importKanji, importerSettingsPage};