import 'package:pigeon/pigeon.dart';

@ConfigurePigeon(
  PigeonOptions(
    dartOut: 'lib/core/messages.g.dart',
    dartOptions: DartOptions(),
    kotlinOut:
        'android/app/src/main/kotlin/com/nicolasnewman/kanji_scanner/Messages.g.kt',
    kotlinOptions: KotlinOptions(),
    dartPackageName: 'kanji_scanner',
  ),
)
//
class GetDecks {
  const GetDecks({required this.decks});
  final Map<int, String> decks;
}

class GetModels {
  const GetModels({required this.models});
  final Map<int, String> models;
}

class ModelInfo {
  final int modelId;
  final String modelName;
  final int noteCount;
  final List<String> fields;
  ModelInfo({
    required this.modelId,
    required this.modelName,
    required this.noteCount,
    required this.fields,
  });
}

class GetModelsWithInfo {
  const GetModelsWithInfo({required this.models});
  final List<ModelInfo> models;
}

class GetModelFields {
  const GetModelFields({required this.fields});
  final List<String> fields;
}

class NoteWithFields {
  final int noteId;
  final int modelId;
  // final Map<String, String> fields;
  final String kanji;
  final List<String> tags;
  NoteWithFields({
    required this.noteId,
    required this.modelId,
    // required this.fields,
    required this.kanji,
    required this.tags,
  });
}

class GetNotesWithFieldsForModel {
  const GetNotesWithFieldsForModel({required this.notes});
  final List<NoteWithFields> notes;
}

class CardInfo {
  final int noteId;
  final int cardOrd;
  final int deckId;
  final String question;
  final String answer;
  final int modelId;
  final String kanji;
  // final Map<String, String> fields;
  final List<String> tags;
  final int reps;
  final int lapses;
  final int type;

  CardInfo({
    required this.noteId,
    required this.cardOrd,
    required this.deckId,
    required this.question,
    required this.answer,
    required this.modelId,
    required this.kanji,
    // required this.fields,
    required this.tags,
    required this.reps,
    required this.lapses,
    required this.type,
  });
}

class GetCardsForModel {
  const GetCardsForModel({required this.cards});
  final List<CardInfo> cards;
}

@HostApi()
abstract class NativeApi {
  GetDecks getDecks();
  GetModels getModels();
  GetModelsWithInfo getModelsWithInfo();
  GetModelFields getModelFields(int modelId);
  GetNotesWithFieldsForModel getNotesWithFieldsForModel(
    int modelId,
    String fieldName,
  );
  GetCardsForModel getCardsForModel(
    int modelId,
    String fieldName,
    int offset,
    int limit,
  );
}
