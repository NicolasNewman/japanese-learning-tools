import { isAssignmentCollection, isSubjectCollection } from "@bachman-dev/wanikani-api-types";
import { Importer } from "../";
import type { KanjiBankData, Level } from "../kanji-bank";

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
  }
}

type WaniKaniKanjiBankData = KanjiBankData<WaniKaniMetadata>

export default class WaniKaniImporter extends Importer<WaniKaniMetadata> {
  private headers;

  constructor(apiKey: string) {
    super();
    this.headers = {
      'Wanikani-Revision': '20170710',
      'Authorization': `Bearer ${apiKey}`,
    }
  }

  serviceLevelToStage(level: number): Level {
    if (level <= 4) {
      return 'Apprentice'
    } else if (level <= 6) {
      return 'Guru'
    } else if (level <= 7) {
      return 'Master'
    } else if (level <= 8) {
      return 'Enlightened'
    } else {
      return 'Burned'
    }
  }

  async load(): Promise<WaniKaniKanjiBankData> {
    let kanjiBankData: WaniKaniKanjiBankData = {};
    let assignmentUrl: string | null = `https://api.wanikani.com/v2/assignments?started=true&subject_types=kanji,vocabulary`;

    while (assignmentUrl) {
      const assignmentsResp = await fetch(assignmentUrl, {
        method: 'GET',
        headers: this.headers,
      })
      if (!assignmentsResp.ok) {
        throw new Error(`Failed to fetch assignments from WaniKani API: ${assignmentsResp.status} ${assignmentsResp.statusText}`);
      }
      const assignments = await assignmentsResp.json();
      if (isAssignmentCollection(assignments)) {
        const subjectIds = assignments.data.map(({ data }) => data.subject_id);
        const assignmentToSrsStage = assignments.data.reduce((prev, { data }) => {
          prev[data.subject_id] = this.serviceLevelToStage(data.srs_stage);
          return prev;
        }, {} as Record<number, Level>);
        const subjectsResp = await fetch(`https://api.wanikani.com/v2/subjects?ids=${subjectIds.join(',')}`, {
          method: 'GET',
          headers: this.headers
        });
        if (!subjectsResp.ok) {
          throw new Error(`Failed to fetch subjects from WaniKani API: ${subjectsResp.status} ${subjectsResp.statusText}`);
        }
        const subjects = await subjectsResp.json();
        if (isSubjectCollection(subjects)) {
          kanjiBankData = {
            ...kanjiBankData,
            ...subjects.data.reduce((prev, { data, object, id }) => {
              // For single character vocabulary, default to the kanji entry if it exists
              if (object === 'kanji' || (object === 'vocabulary' && !prev[data.characters])) {
                prev[data.characters] = {
                  level: assignmentToSrsStage[id],
                  source: 'wanikani',
                  type: object,
                  meaning: data.meanings.map(m => m.meaning).join(', '),
                  metadata: {
                    url: data.document_url,
                    level: data.level.toString()
                  }
                }
                if (object === 'kanji') {
                  prev[data.characters].metadata.kanjiData = {
                    onyomiReadings: data.readings.filter(r => r.type === 'onyomi').map(r => r.reading),
                    kunyomiReadings: data.readings.filter(r => r.type === 'kunyomi').map(r => r.reading),
                    nanoriReadings: data.readings.filter(r => r.type === 'nanori').map(r => r.reading)
                  };
                } else if (object === 'vocabulary') {
                  prev[data.characters].metadata.vocabularyData = {
                    partsOfSpeech: data.parts_of_speech
                  };
                }
              }
              return prev;
            }, {} as WaniKaniKanjiBankData)
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