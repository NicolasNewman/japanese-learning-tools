import 'package:flutter/material.dart';
import 'package:kanji_scanner/core/utils.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';

class KanjiBankText extends StatelessWidget {
  final String text;
  final KanjiBankData kanjiBank;
  const KanjiBankText({super.key, required this.text, required this.kanjiBank});

  @override
  Widget build(BuildContext context) {
    final bool containsKanji = hasKanji(text);
    final bool inKanjiBank = kanjiBank[text] != null ? true : false;
    final bool isVocabulary =
        inKanjiBank && kanjiBank[text]!.type == KanjiType.vocabulary;
    if (containsKanji && inKanjiBank && isVocabulary) {
      return Text(text, style: const TextStyle(color: Colors.purple));
    } else if (containsKanji && inKanjiBank) {
      return Text(text, style: const TextStyle(color: Colors.pink));
    } else if (containsKanji) {
      return Text.rich(
        TextSpan(
          children: text.split('').map((char) {
            final entry = kanjiBank[char];
            final color = (entry != null)
                ? (entry.type == KanjiType.vocabulary
                      ? Colors.purple
                      : Colors.pink)
                : Theme.of(context).colorScheme.onSurface;
            return TextSpan(
              text: char,
              style: TextStyle(color: color),
            );
          }).toList(),
        ),
      );
    }
    return Text(
      text,
      style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
    );
  }
}
