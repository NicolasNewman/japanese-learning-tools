import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';

class KanjiData {
  final List<String> onyomiReadings;
  final List<String> kunyomiReadings;
  final List<String> nanoriReadings;

  KanjiData({
    required this.onyomiReadings,
    required this.kunyomiReadings,
    required this.nanoriReadings,
  });
}

class VocabularyReading {
  final String reading;
  final bool primary;

  VocabularyReading({required this.reading, required this.primary});

  factory VocabularyReading.fromJson(Map<String, dynamic> json) {
    return VocabularyReading(
      reading: json['reading'] as String,
      primary: json['primary'] as bool,
    );
  }
}

class VocabularyData {
  final List<String> partsOfSpeech;
  final List<VocabularyReading> readings;

  VocabularyData({required this.partsOfSpeech, required this.readings});
}

class WaniKaniMetadata {
  final String url;
  final String level;
  final KanjiData? kanjiData;
  final VocabularyData? vocabularyData;

  WaniKaniMetadata({
    required this.url,
    required this.level,
    this.kanjiData,
    this.vocabularyData,
  });

  Map<String, dynamic> toJson() {
    return {
      'url': url,
      'level': level,
      'kanjiData': kanjiData != null
          ? {
              'onyomiReadings': kanjiData!.onyomiReadings,
              'kunyomiReadings': kanjiData!.kunyomiReadings,
              'nanoriReadings': kanjiData!.nanoriReadings,
            }
          : null,
      'vocabularyData': vocabularyData != null
          ? {
              'partsOfSpeech': vocabularyData!.partsOfSpeech,
              'readings': vocabularyData!.readings
                  .map((r) => {'reading': r.reading, 'primary': r.primary})
                  .toList(),
            }
          : null,
    };
  }

  factory WaniKaniMetadata.fromJson(Map<String, dynamic> json) {
    KanjiData? kanjiData;
    if (json['kanjiData'] != null) {
      final kanjiJson = json['kanjiData'] as Map<String, dynamic>;
      kanjiData = KanjiData(
        onyomiReadings: (kanjiJson['onyomiReadings'] as List).cast<String>(),
        kunyomiReadings: (kanjiJson['kunyomiReadings'] as List).cast<String>(),
        nanoriReadings: (kanjiJson['nanoriReadings'] as List).cast<String>(),
      );
    }

    VocabularyData? vocabularyData;
    if (json['vocabularyData'] != null) {
      final vocabJson = json['vocabularyData'] as Map<String, dynamic>;
      vocabularyData = VocabularyData(
        partsOfSpeech: (vocabJson['partsOfSpeech'] as List).cast<String>(),
        readings: (vocabJson['readings'] as List)
            .map((r) => VocabularyReading.fromJson(r as Map<String, dynamic>))
            .toList(),
      );
    }

    return WaniKaniMetadata(
      url: json['url'] as String,
      level: json['level'] as String,
      kanjiData: kanjiData,
      vocabularyData: vocabularyData,
    );
  }
}

typedef WaniKaniKanjiBankData = KanjiBankData<WaniKaniMetadata>;
