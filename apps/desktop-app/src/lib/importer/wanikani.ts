import { isAssignmentCollection, isSubjectCollection } from "@bachman-dev/wanikani-api-types";
import { Importer } from ".";
import type { KanjiBankData } from "../kanji-bank";

export default class WaniKaniImporter extends Importer {
  readonly headers = {
    'Wanikani-Revision': '20170710',
    // 'Authorization': `Bearer ${this.options.apiKey}`,
  };

  async load(): Promise<KanjiBankData> {
    let kanjiBankData: KanjiBankData = {};
    let assignmentUrl: string | null = `https://api.wanikani.com/v2/assignments?started=true&subject_types=kanji,vocabulary`;

    while (assignmentUrl) {
      const assignments = await fetch(assignmentUrl, {
        method: 'GET',
        headers: this.headers,
      })
      if (isAssignmentCollection(assignments)) {
        const subjectIds = assignments.data.map(({ data }) => data.subject_id);
        const subjects = await fetch(`https://api.wanikani.com/v2/subjects?ids=${subjectIds.join(',')}`, {
          method: 'GET',
          headers: this.headers
        });
        if (isSubjectCollection(subjects)) {
          kanjiBankData = {
            ...kanjiBankData,
            ...subjects.data.reduce((prev, { data, object }) => {
              if (object === 'kanji' || object === 'vocabulary') {
                prev[data.characters] = {
                  level: data.level,
                  source: 'wanikani',
                  type: object,
                }
              }
              return prev;
            }, {} as KanjiBankData)
          }
          assignmentUrl = assignments.pages.next_url || null;
        } else {
          throw new Error('Failed to fetch subjects from WaniKani API');
        }
      } else {
        throw new Error('Failed to fetch assignments from WaniKani API');
      }
    }
    return kanjiBankData
  }
}