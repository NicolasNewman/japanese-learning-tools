import { KanjiBankData } from "src/index.js";
import type { VocabularyReading } from "@bachman-dev/wanikani-api-types";

export type WaniKaniMetadata = {
    url: string;
    level: string;
    kanjiData?: {
        onyomiReadings: string[];
        kunyomiReadings: string[];
        nanoriReadings: string[];
    };
    vocabularyData?: {
        partsOfSpeech: string[];
        readings: VocabularyReading[];
    }
}

export type WaniKaniKanjiBankData = KanjiBankData<WaniKaniMetadata>