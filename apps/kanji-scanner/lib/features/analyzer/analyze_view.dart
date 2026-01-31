import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/features/analyzer/widgets/jisho.dart';
import 'package:kanji_scanner/features/analyzer/widgets/list_view.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:kanji_scanner/shared/providers/state.dart';
import 'package:kanji_scanner/src/rust/api/sudachi_api.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'analyze_view.g.dart';

class AnalyzerView extends ConsumerStatefulWidget {
  @override
  ConsumerState<AnalyzerView> createState() => _AnalyzerViewState();

  const AnalyzerView({super.key});
}

enum SwipeAction { jisho, anki }

@riverpod
Future<(List<TokenInfo>?, KanjiBankData)> analyzedData(Ref ref) async {
  final parsedSentence = await ref.watch(parsedSentenceProvider.future);
  final kanjiBank = await ref.watch(kanjiBankProvider.future);
  return (parsedSentence, kanjiBank);
}

class _AnalyzerViewState extends ConsumerState<AnalyzerView> {
  String? _selectedItem;
  SwipeAction? _action;

  void triggerJisho(String term) {
    setState(() {
      _selectedItem = term;
      _action = SwipeAction.jisho;
    });
  }

  void triggerAnki(String term) {
    setState(() {
      _selectedItem = term;
      _action = SwipeAction.anki;
    });
  }

  void clearSelection() {
    setState(() {
      _selectedItem = null;
      _action = null;
    });
  }

  @override
  Widget build(BuildContext context) {
    final rawSentence = ref.watch(rawSentenceProvider);
    final data = ref.watch(analyzedDataProvider);

    if (rawSentence == null) {
      return Center(
        child: Text('No sentence selected. Tap on text to analyze.'),
      );
    }

    return data.when(
      data: (tuple) {
        final (parsedSentence, kanjiBank) = tuple;
        return Card(
          shadowColor: Colors.transparent,
          margin: const EdgeInsets.all(8.0),
          child: Scaffold(
            body: _action == null
                ? ListViewWidget(
                    parsedSentence: parsedSentence,
                    kanjiBank: kanjiBank,
                    triggerJisho: triggerJisho,
                    triggerAnki: triggerAnki,
                  )
                : (_action == SwipeAction.jisho
                      ? JishoFrame(
                          searchTerm: _selectedItem!,
                          clearSelection: clearSelection,
                        )
                      : Text("TODO: Anki integration")),
          ),
        );
      },
      loading: () => CircularProgressIndicator(),
      error: (error, stack) => Text('Error: $error'),
    );
  }
}
