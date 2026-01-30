import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:kanji_scanner/shared/models/kanji/wanikani.dart';

class WaniKaniService {
  final String apiKey;
  late final Map<String, String> headers;

  WaniKaniService(this.apiKey) {
    headers = {
      'Wanikani-Revision': '20170710',
      'Authorization': 'Bearer $apiKey',
    };
  }

  Future<WaniKaniKanjiBankData> load() async {
    WaniKaniKanjiBankData kanjiBankData = {};
    String? assignmentUrl =
        'https://api.wanikani.com/v2/assignments?started=true&subject_types=kanji,vocabulary';

    while (assignmentUrl != null) {
      final assignmentsResp = await http.get(
        Uri.parse(assignmentUrl),
        headers: headers,
      );

      if (assignmentsResp.statusCode != 200) {
        throw Exception(
          'Failed to fetch assignments from WaniKani API: ${assignmentsResp.statusCode} ${assignmentsResp.reasonPhrase}',
        );
      }

      final assignments =
          jsonDecode(assignmentsResp.body) as Map<String, dynamic>;

      if (!_isAssignmentCollection(assignments)) {
        throw Exception('Failed to fetch assignments from WaniKani API');
      }

      final data = assignments['data'] as List;
      final subjectIds = data
          .map((item) => item['data']['subject_id'] as int)
          .toList();

      final assignmentToSrsStage = <int, Level>{};
      for (final item in data) {
        final itemData = item['data'] as Map<String, dynamic>;
        assignmentToSrsStage[itemData['subject_id'] as int] =
            Level.fromSrsStage(itemData['srs_stage'] as int);
      }

      final subjectsResp = await http.get(
        Uri.parse(
          'https://api.wanikani.com/v2/subjects?ids=${subjectIds.join(',')}',
        ),
        headers: headers,
      );

      if (subjectsResp.statusCode != 200) {
        throw Exception(
          'Failed to fetch subjects from WaniKani API: ${subjectsResp.statusCode} ${subjectsResp.reasonPhrase}',
        );
      }

      final subjects = jsonDecode(subjectsResp.body) as Map<String, dynamic>;

      if (!_isSubjectCollection(subjects)) {
        throw Exception('Failed to fetch subjects from WaniKani API');
      }

      final subjectsData = subjects['data'] as List;
      for (final subject in subjectsData) {
        final subjectData = subject['data'] as Map<String, dynamic>;
        final objectType = subject['object'] as String;
        final id = subject['id'] as int;
        final characters = subjectData['characters'] as String;

        // For single character vocabulary, default to the kanji entry if it exists
        if (objectType == 'kanji' ||
            (objectType == 'vocabulary' &&
                !kanjiBankData.containsKey(characters))) {
          final meanings = (subjectData['meanings'] as List)
              .map((m) => m['meaning'] as String)
              .join(', ');

          WaniKaniMetadata metadata = WaniKaniMetadata(
            url: subjectData['document_url'] as String,
            level: subjectData['level'].toString(),
          );

          if (objectType == 'kanji') {
            final readings = subjectData['readings'] as List;
            metadata = WaniKaniMetadata(
              url: metadata.url,
              level: metadata.level,
              kanjiData: KanjiData(
                onyomiReadings: readings
                    .where((r) => r['type'] == 'onyomi')
                    .map((r) => r['reading'] as String)
                    .toList(),
                kunyomiReadings: readings
                    .where((r) => r['type'] == 'kunyomi')
                    .map((r) => r['reading'] as String)
                    .toList(),
                nanoriReadings: readings
                    .where((r) => r['type'] == 'nanori')
                    .map((r) => r['reading'] as String)
                    .toList(),
              ),
            );
          } else if (objectType == 'vocabulary') {
            final readings = (subjectData['readings'] as List)
                .map(
                  (r) => VocabularyReading.fromJson(r as Map<String, dynamic>),
                )
                .toList();

            readings.sort((a, b) {
              final aNum = a.primary ? 1 : 0;
              final bNum = b.primary ? 1 : 0;
              return bNum - aNum;
            });

            metadata = WaniKaniMetadata(
              url: metadata.url,
              level: metadata.level,
              vocabularyData: VocabularyData(
                partsOfSpeech: (subjectData['parts_of_speech'] as List)
                    .cast<String>(),
                readings: readings,
              ),
            );
          }

          kanjiBankData[characters] = KanjiBankEntry(
            level: assignmentToSrsStage[id]!,
            source: KanjiSource.wanikani,
            type: objectType,
            meaning: meanings,
            metadata: metadata,
          );
        }
      }

      final pages = assignments['pages'] as Map<String, dynamic>?;
      assignmentUrl = pages?['next_url'] as String?;
    }
    return kanjiBankData;
  }

  bool _isAssignmentCollection(Map<String, dynamic> json) {
    return json.containsKey('data') && json.containsKey('pages');
  }

  bool _isSubjectCollection(Map<String, dynamic> json) {
    return json.containsKey('data');
  }
}
