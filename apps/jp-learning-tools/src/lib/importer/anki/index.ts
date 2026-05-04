import AnkiConnectClient from "$lib/ankiApi";
import { Importer } from "../";
import type {
  AnkiMetadata,
  AnkiKanjiBankData,
  KanjiBankData,
  Level,
} from "@nicolasnewman/kanji-bank-types";
import { YankiConnect } from "yanki-connect";
import type { AnkiSettingsStore } from "./store";

export default class AnkiImporter extends Importer<AnkiMetadata> {
  private models: NonNullable<AnkiSettingsStore["syncedModels"]>;
  private deckNames: string[];
  version = 1;

  constructor(
    models: NonNullable<AnkiSettingsStore["syncedModels"]>,
    deckNames: string[],
  ) {
    super();
    this.models = models;
    this.deckNames = deckNames;
  }

  async load(): Promise<KanjiBankData<AnkiMetadata>> {
    const cards = await AnkiConnectClient.getCards(this.models, this.deckNames);

    return Object.entries(cards).reduce((prev, [def, data]) => {
      return {
        ...prev,
        [def]: {
          ...data,
          level: this.serviceLevelToStage(data.metadata.stats),
        },
      };
    }, {} as KanjiBankData<AnkiMetadata>);
  }

  serviceLevelToStage(level: AnkiMetadata["stats"]): Level {
    if (level.reps === 0) {
      return "Apprentice";
    } else if (level.interval < 21) {
      return "Guru";
    } else if (level.interval < 111) {
      return "Master";
    } else if (level.interval < 365) {
      return "Enlightened";
    } else {
      return "Burned";
    }
  }
}
