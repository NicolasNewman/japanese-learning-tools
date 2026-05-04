import {
  containsKanji,
  type AnkiMetadata,
  type KanjiBankEntry,
} from "@nicolasnewman/kanji-bank-types";
import { YankiConnect } from "yanki-connect";
import type { AnkiSettingsStore } from "./importer/anki/store";

class DeckTreeNode {
  name: string;
  children: Record<string, DeckTreeNode>;
  //   parent: DeckTreeNode
  __isSelected: boolean;

  constructor(name: string) {
    this.name = name;
    this.children = {};
    this.__isSelected = false;
  }
}

export default class AnkiConnectClient {
  private static instance: null | YankiConnect = null;

  public static getInstance = () => {
    if (AnkiConnectClient.instance == null) {
      AnkiConnectClient.instance = new YankiConnect();
    }
    return AnkiConnectClient.instance;
  };

  public static getInitialInfo = async () => {
    const client = AnkiConnectClient.getInstance();

    const decks = await client.deck.deckNames();

    const modelNames = await client.model.modelNames();

    const models: Record<string, { count: number; fields: string[] }> = {};
    for (const modelName of modelNames) {
      const noteCount = (
        await client.note.findNotes({ query: `note:"${modelName}"` })
      ).length;
      const fields = await client.model.modelFieldNames({ modelName });
      models[modelName] = {
        count: noteCount,
        fields,
      };
    }

    return {
      decks,
      models,
    };
  };

  public static getCards = async (
    models: NonNullable<AnkiSettingsStore["syncedModels"]>,
    deckNames: string[],
  ) => {
    const client = AnkiConnectClient.getInstance();

    let cardIds: number[] = [];
    for (const modelName of Object.keys(models)) {
      cardIds = [
        ...cardIds,
        ...(await client.card.findCards({
          query: `note:"${modelName}" is:review (${deckNames
            .map((deck) => `deck:"${deck}"`)
            .join(" OR ")})`,
        })),
      ];
    }

    const cards = (await client.card.cardsInfo({ cards: cardIds })).reduce(
      (prev, card) => {
        const kanjiFieldName = models[card.modelName].kanji;
        const fieldValue = (card.fields[kanjiFieldName]?.value ?? "")
          .replace(/\[.*\]/g, "")
          .replace(/\<.*\>/g, "")
          // TODO: this is a hack to handle cases where the field contains multiple values separated by commas. We should ideally allow users to customize this behavior.
          .split(",")
          .filter((str) => containsKanji(str));

        const meaningFieldName = models[card.modelName].meaning;
        const meaningValue = card.fields[meaningFieldName]?.value ?? "";

        const obj = {
          source: "anki",
          metadata: {
            modelName: card.modelName,
            cardId: card.cardId,
            deckName: card.deckName,
            stats: {
              interval: card.interval,
              reps: card.reps,
              lapses: card.lapses,
            },
            fields: Object.fromEntries(
              Object.entries(card.fields).map(([key, value]) => [
                key,
                value.value,
              ]),
            ),
          },
        } as Omit<KanjiBankEntry<AnkiMetadata>, "level" | "type">;
        return {
          ...prev,
          ...fieldValue.reduce((prev, value) => {
            prev[value] = {
              ...obj,
              type: value.length === 1 ? "kanji" : "vocabulary",
              meaning: meaningValue,
            };
            return prev;
          }, {} as Record<string, Omit<KanjiBankEntry<AnkiMetadata>, "level">>),
        };
        // card.fields[models[card.modelName]
      },
      {} as Record<string, Omit<KanjiBankEntry<AnkiMetadata>, "level">>,
    );

    return cards;
  };
}
