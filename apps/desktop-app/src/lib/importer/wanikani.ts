import { Importer } from ".";
import type { KanjiBankData } from "../kanji-bank";

export default class WaniKaniImporter extends Importer<'wanikani'> {
  load(): Promise<KanjiBankData> {
      throw new Error("Method not implemented.");
  }
}