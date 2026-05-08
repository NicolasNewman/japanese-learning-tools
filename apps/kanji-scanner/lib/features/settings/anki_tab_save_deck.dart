import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/core/messages.g.dart';
import 'package:kanji_scanner/services/api/anki_service.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';

final deckProvider = FutureProvider<Map<String, int>>((ref) async {
  return await AnkiService().getDecks();
});

class AnkiTabSentenceDeckWidget extends ConsumerStatefulWidget {
  const AnkiTabSentenceDeckWidget({super.key, required this.sortedModels});

  final List<ModelInfo> sortedModels;

  @override
  ConsumerState<AnkiTabSentenceDeckWidget> createState() =>
      _AnkiTabSentenceDeckState();
}

class _AnkiTabSentenceDeckState
    extends ConsumerState<AnkiTabSentenceDeckWidget> {
  @override
  Widget build(BuildContext context) {
    final decks = ref.watch(deckProvider);

    final targetDeckAsync = ref.watch(ankiTargetDeckProvider);

    final targetDeck = targetDeckAsync.asData?.value;

    return decks.when(
      data: (decks) => SizedBox(
        width: 250,
        child: Column(
          spacing: 16,
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(width: 16),
            Row(
              spacing: 16,
              children: [
                Text(
                  "Deck: ",
                  style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
                ),
                Expanded(
                  child: DropdownMenu(
                    initialSelection: targetDeck?.deckId,
                    dropdownMenuEntries: decks.entries
                        .map(
                          (entry) => DropdownMenuEntry(
                            value: entry.value,
                            label: entry.key,
                          ),
                        )
                        .toList(),
                    onSelected: (value) {
                      if (value != null) {
                        ref
                            .read(ankiTargetDeckProvider.notifier)
                            .updateDeck(value);
                      }
                    },
                    hintText: "Select Deck",
                  ),
                ),
              ],
            ),
            Row(
              spacing: 16,
              children: [
                Text(
                  "Card Model: ",
                  style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
                ),
                Expanded(
                  child: DropdownMenu(
                    initialSelection: targetDeck?.modelId,
                    dropdownMenuEntries: widget.sortedModels
                        .map(
                          (model) => DropdownMenuEntry(
                            value: model.modelId,
                            label: model.modelName,
                          ),
                        )
                        .toList(),
                    onSelected: (value) {
                      if (value != null) {
                        ref
                            .read(ankiTargetDeckProvider.notifier)
                            .updateModel(value);
                      }
                    },
                    hintText: "Select Model",
                  ),
                ),
              ],
            ),
            Row(
              spacing: 16,
              children: [
                Text(
                  "Kanji Field: ",
                  style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
                ),
                Expanded(
                  child: DropdownMenu(
                    initialSelection: targetDeck?.fieldKanji,
                    enabled: targetDeck?.modelId != null,
                    width: 200,
                    dropdownMenuEntries: widget.sortedModels
                        .firstWhere(
                          (model) => model.modelId == targetDeck?.modelId,
                          orElse: () => ModelInfo(
                            modelId: 0,
                            modelName: '',
                            fields: [],
                            noteCount: 0,
                          ),
                        )
                        .fields
                        .map(
                          (field) =>
                              DropdownMenuEntry(value: field, label: field),
                        )
                        .toList(),
                    onSelected: (value) {
                      if (value != null) {
                        ref
                            .read(ankiTargetDeckProvider.notifier)
                            .updateFieldKanji(value);
                      }
                    },
                    hintText: "Select Kanji Field",
                  ),
                ),
              ],
            ),
            Row(
              spacing: 16,
              children: [
                Text(
                  "Sentence Field: ",
                  style: TextStyle(fontWeight: FontWeight.bold, fontSize: 16),
                ),
                Expanded(
                  child: DropdownMenu(
                    initialSelection: targetDeck?.fieldSentence,
                    enabled: targetDeck?.modelId != null,
                    width: 200,
                    dropdownMenuEntries: widget.sortedModels
                        .firstWhere(
                          (model) => model.modelId == targetDeck?.modelId,
                          orElse: () => ModelInfo(
                            modelId: 0,
                            modelName: '',
                            fields: [],
                            noteCount: 0,
                          ),
                        )
                        .fields
                        .map(
                          (field) =>
                              DropdownMenuEntry(value: field, label: field),
                        )
                        .toList(),
                    onSelected: (value) {
                      if (value != null) {
                        ref
                            .read(ankiTargetDeckProvider.notifier)
                            .updateFieldSentence(value);
                      }
                    },
                    hintText: "Select Sentence Field",
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
      error: (error, stack) => Text('Error loading decks: $error'),
      loading: () => CircularProgressIndicator(),
    );
  }
}
