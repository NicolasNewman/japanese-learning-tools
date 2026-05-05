import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:kanji_scanner/core/messages.g.dart';
import 'package:kanji_scanner/services/api/anki_service.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/models/kanji/anki.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';

final modelProvider = FutureProvider<List<ModelInfo>>((ref) async {
  return await AnkiService().getModelsWithInfo();
});

class AnkiTab extends ConsumerStatefulWidget {
  const AnkiTab({super.key, required this.getController});

  final TextEditingController Function(String) getController;

  @override
  ConsumerState<AnkiTab> createState() => _AnkiTabState();
}

class _AnkiTabState extends ConsumerState<AnkiTab> {
  bool _isLoading = false;

  Level _convertAnkiTypeToLevel(int type, int reps, int lapses) {
    if (reps == 0) return Level.apprentice;
    if (reps < 5) return Level.apprentice;
    if (reps < 10) return Level.guru;
    if (reps < 20) return Level.master;
    return Level.enlightened;
  }

  @override
  Widget build(BuildContext context) {
    final lastUpdated = ref.watch(ankiLastUpdatedProvider);
    final selectedModelsAsync = ref.watch(ankiSelectedModelsProvider);
    final models = ref.watch(modelProvider);

    return models.when(
      data: (data) {
        final sortedModels = data
          ..sort((a, b) => b.noteCount.compareTo(a.noteCount));
        print(
          'Loaded Anki models: ${sortedModels.map((m) => m.modelName).join(", ")}',
        );

        final selectedModels =
            selectedModelsAsync.asData?.value.toSet() ?? <ModelSelection>{};

        print('Selected model IDs: $selectedModels');
        return Center(
          child: SizedBox(
            width: 250,
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Expanded(
                  child: Scrollbar(
                    child: ListView.separated(
                      itemCount: sortedModels.length,
                      separatorBuilder: (ctx, idx) => Divider(),
                      itemBuilder: (ctx, idx) {
                        final model = sortedModels[idx];
                        final isSelected = ref
                            .read(ankiSelectedModelsProvider.notifier)
                            .current
                            .any((m) => m.modelId == model.modelId);

                        return ListTile(
                          title: Text(model.modelName),
                          subtitle: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text('Notes: ${model.noteCount}'),
                              isSelected
                                  ? DropdownMenu(
                                      enabled: !_isLoading,
                                      dropdownMenuEntries: model.fields
                                          .map(
                                            (field) => DropdownMenuEntry(
                                              value: field,
                                              label: field,
                                            ),
                                          )
                                          .toList(),
                                      onSelected: (value) {
                                        if (value != null) {
                                          ref
                                              .read(
                                                ankiSelectedModelsProvider
                                                    .notifier,
                                              )
                                              .setField(model.modelId, value);
                                        }
                                      },
                                      initialSelection: ref
                                          .read(
                                            ankiSelectedModelsProvider.notifier,
                                          )
                                          .current
                                          .firstWhere(
                                            (m) => m.modelId == model.modelId,
                                            orElse: () => ModelSelection(
                                              modelId: model.modelId,
                                            ),
                                          )
                                          .selectedField,
                                      hintText: "Field",
                                    )
                                  : SizedBox.shrink(),
                            ],
                          ),
                          leading: Checkbox(
                            value: isSelected,
                            onChanged: _isLoading
                                ? null
                                : (value) {
                                    ref
                                        .read(
                                          ankiSelectedModelsProvider.notifier,
                                        )
                                        .toggleModel(model.modelId);
                                  },
                          ),
                        );
                      },
                    ),
                  ),
                ),
                Row(
                  children: [
                    FilledButton(
                      onPressed: _isLoading
                          ? null
                          : () async {
                              showDialog(
                                context: context,
                                barrierDismissible: false,
                                builder: (context) => AlertDialog(
                                  content: Column(
                                    mainAxisSize: MainAxisSize.min,
                                    children: [
                                      CircularProgressIndicator(),
                                      SizedBox(height: 16),
                                      Text('Loading AnkiDroid cards...'),
                                    ],
                                  ),
                                ),
                              );

                              setState(() {
                                _isLoading = true;
                              });

                              await Future.delayed(
                                Duration(milliseconds: 1000),
                              );

                              try {
                                final kanjiBankData =
                                    <String, KanjiBankEntry<AnkiMetadata>>{};

                                for (var model in selectedModels) {
                                  if (model.selectedField == null) continue;

                                  const batchSize = 500;
                                  int offset = 0;
                                  bool hasMore = true;

                                  while (hasMore) {
                                    var batch = await AnkiService()
                                        .getCardsForModel(
                                          model.modelId,
                                          model.selectedField!,
                                          offset: offset,
                                          limit: batchSize,
                                        );

                                    for (var card in batch) {
                                      kanjiBankData[card.kanji] =
                                          KanjiBankEntry(
                                            level: _convertAnkiTypeToLevel(
                                              card.type,
                                              card.reps,
                                              card.lapses,
                                            ),
                                            type: KanjiType.vocabulary,
                                            source: KanjiSource.anki,
                                            meaning: '', // TODO: implement
                                            metadata: AnkiMetadata(
                                              noteId: card.noteId,
                                              modelId: card.modelId,
                                              reps: card.reps,
                                              lapses: card.lapses,
                                              type: card.type,
                                            ),
                                          );
                                    }

                                    offset += batchSize;
                                    hasMore = batch.length == batchSize;

                                    await Future.delayed(
                                      Duration(milliseconds: 50),
                                    );
                                  }
                                }

                                await ref.read(kanjiBankProvider.future);
                                ref
                                    .read(kanjiBankProvider.notifier)
                                    .updateKanjiBankData(kanjiBankData);

                                ref
                                    .read(ankiLastUpdatedProvider.notifier)
                                    .setLastUpdated(
                                      DateFormat(
                                        'yyyy-MM-dd HH:mm:ss',
                                      ).format(DateTime.now()),
                                    );
                              } catch (e) {
                                print('Error loading Anki data: $e');
                              } finally {
                                setState(() {
                                  _isLoading = false;
                                });

                                if (context.mounted) {
                                  Navigator.of(context).pop();
                                }
                              }
                            },
                      child: _isLoading
                          ? SizedBox(
                              height: 24,
                              width: 24,
                              child: CircularProgressIndicator(
                                color: Colors.white,
                                strokeWidth: 2.0,
                              ),
                            )
                          : Text("Update"),
                    ),
                    SizedBox(width: 16),
                    Text("Last updated: \n ${lastUpdated.value}"),
                  ],
                ),
              ],
            ),
          ),
        );
      },
      error: (error, stack) {
        return Center(child: Text('Error loading Anki decks: $error'));
      },
      loading: () {
        return Center(child: CircularProgressIndicator());
      },
    );
  }
}
