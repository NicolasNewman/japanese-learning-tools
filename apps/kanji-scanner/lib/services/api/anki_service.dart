import 'package:kanji_scanner/core/messages.g.dart';

class AnkiService {
  final NativeApi _api = NativeApi();
  AnkiService() {}

  Future<Map<String, int>> getDecks() async {
    final result = await _api.getDecks();
    return result.decks.map((k, v) => MapEntry(v, k));
  }

  Future<Map<String, int>> getModels() async {
    final result = await _api.getModels();
    return result.models.map((k, v) => MapEntry(v, k));
  }

  Future<List<ModelInfo>> getModelsWithInfo() async {
    final result = await _api.getModelsWithInfo();
    return result.models;
  }

  Future<List<String>> getModelFields(int modelId) async {
    final result = await _api.getModelFields(modelId);
    return result.fields;
  }

  Future<List<NoteWithFields>> getNotesWithFieldsForModel(
    int modelId,
    String fieldName,
  ) async {
    final result = await _api.getNotesWithFieldsForModel(modelId, fieldName);
    return result.notes;
  }

  Future<List<CardInfo>> getCardsForModel(
    int modelId,
    String fieldName, {
    int offset = 0,
    int limit = 1000,
  }) async {
    final result = await _api.getCardsForModel(
      modelId,
      fieldName,
      offset,
      limit,
    );
    return result.cards;
  }
}
