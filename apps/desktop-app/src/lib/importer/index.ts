import type { KanjiBankData, KanjiBankSource } from "../kanji-bank";

abstract class Importer<T extends KanjiBankSource> {
  abstract load(): Promise<KanjiBankData>;
}

const importKanji = async (): Promise<KanjiBankData> => {
    const WaniKaniImporter = (await import("./wanikani")).default;

    return {
        ...(await new WaniKaniImporter().load()),
    }
}

export { Importer }
export default importKanji;