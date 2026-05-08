import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/core/messages.g.dart';
import 'package:kanji_scanner/features/settings/anki_tab_kanji_model.dart';
import 'package:kanji_scanner/features/settings/anki_tab_save_deck.dart';
import 'package:kanji_scanner/services/api/anki_service.dart';

final modelProvider = FutureProvider<List<ModelInfo>>((ref) async {
  return await AnkiService().getModelsWithInfo();
});

class AnkiTab extends ConsumerStatefulWidget {
  const AnkiTab({super.key, required this.getController});

  final TextEditingController Function(String) getController;

  @override
  ConsumerState<AnkiTab> createState() => _AnkiTabState();
}

class _AnkiTabState extends ConsumerState<AnkiTab>
    with TickerProviderStateMixin {
  late final _tabController = TabController(length: 2, vsync: this);

  @override
  Widget build(BuildContext context) {
    final models = ref.watch(modelProvider);

    return models.when(
      data: (data) {
        final sortedModels = data
          ..sort((a, b) => b.noteCount.compareTo(a.noteCount));

        return Column(
          children: [
            TabBar.secondary(
              controller: _tabController,
              tabs: [
                Tab(text: 'Sentence Deck'),
                Tab(text: 'Import Models'),
              ],
            ),
            Expanded(
              child: TabBarView(
                controller: _tabController,
                children: [
                  AnkiTabSentenceDeckWidget(sortedModels: sortedModels),
                  AnkiTabKanjiModelWidget(sortedModels: sortedModels),
                ],
              ),
            ),
          ],
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
