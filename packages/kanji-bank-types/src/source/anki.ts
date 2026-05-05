import { KanjiBankData } from "src/index.js";

export type AnkiMetadata = {
  cardId: number;
  modelName: string;
  deckName: string;
  stats: {
    interval: number;
    reps: number;
    lapses: number;
  };
  fields: Record<string, string>;
};

export type AnkiKanjiBankData = KanjiBankData<AnkiMetadata>;
