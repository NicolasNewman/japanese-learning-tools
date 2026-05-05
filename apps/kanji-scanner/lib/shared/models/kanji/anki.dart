import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';

class AnkiMetadata {
  final int noteId;
  final int modelId;
  final int reps;
  final int lapses;
  final int type;

  AnkiMetadata({
    required this.noteId,
    required this.modelId,
    required this.reps,
    required this.lapses,
    required this.type,
  });

  Map<String, dynamic> toJson() {
    return {
      'noteId': noteId,
      'modelId': modelId,
      'reps': reps,
      'lapses': lapses,
      'type': type,
    };
  }

  factory AnkiMetadata.fromJson(Map<String, dynamic> json) {
    return AnkiMetadata(
      noteId: json['noteId'] as int,
      modelId: json['modelId'] as int,
      reps: json['reps'] as int,
      lapses: json['lapses'] as int,
      type: json['type'] as int,
    );
  }
}

typedef AnkiKanjiBankData = KanjiBankData<AnkiMetadata>;
