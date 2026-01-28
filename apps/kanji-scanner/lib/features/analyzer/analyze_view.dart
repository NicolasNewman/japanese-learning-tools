import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/features/analyzer/widgets/jisho.dart';
import 'package:kanji_scanner/features/analyzer/widgets/list_view.dart';
import 'package:kanji_scanner/shared/providers/state.dart';

class AnalyzerView extends ConsumerStatefulWidget {
  @override
  ConsumerState<AnalyzerView> createState() => _AnalyzerViewState();

  const AnalyzerView({super.key});
}

enum SwipeAction { jisho, anki }

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
    final parsedSentence = ref.watch(parsedSentenceProvider);

    if (rawSentence == null) {
      return Center(
        child: Text('No sentence selected. Tap on text to analyze.'),
      );
    }

    return parsedSentence.when(
      data: (parsedSentence) {
        return Card(
          shadowColor: Colors.transparent,
          margin: const EdgeInsets.all(8.0),
          child: Scaffold(
            body: _action == null
                ? ListViewWidget(
                    parsedSentence: parsedSentence,
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
