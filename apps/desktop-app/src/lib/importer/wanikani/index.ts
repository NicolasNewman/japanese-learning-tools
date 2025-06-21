import { isAssignmentCollection, isSubjectCollection } from "@bachman-dev/wanikani-api-types";
import { Importer } from "../";
import type { KanjiBankData, Level } from "../kanji-bank";

export default class WaniKaniImporter extends Importer {
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

  async load(): Promise<KanjiBankData> {
    let kanjiBankData: KanjiBankData = {};
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
              if (object === 'kanji' || object === 'vocabulary') {
                prev[data.characters] = {
                  level: assignmentToSrsStage[id],
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