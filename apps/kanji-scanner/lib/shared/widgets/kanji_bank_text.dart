import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:kanji_scanner/core/utils.dart';
import 'package:kanji_scanner/shared/models/enums.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:url_launcher/url_launcher.dart';

class KanjiBankTextLinkConfig {
  final bool linkKnown;
  final bool linkUnknown;
  final bool addSpace;
  final DictionaryBackendType backend;

  const KanjiBankTextLinkConfig({
    required this.backend,
    this.linkKnown = true,
    this.linkUnknown = false,
    this.addSpace = true,
  }) : assert(
         linkKnown || linkUnknown,
         'At least one of linkKnown or linkUnknown must be true',
       );
}

class KanjiBankText extends StatelessWidget {
  final String text;
  final KanjiBankData kanjiBank;
  final KanjiBankTextLinkConfig? linkConfig;
  const KanjiBankText({
    super.key,
    required this.text,
    required this.kanjiBank,
    this.linkConfig,
  });

  @override
  Widget build(BuildContext context) {
    return RichText(text: toTextSpan(context));
  }

  TapGestureRecognizer _getTapRecognizer(String text) {
    return TapGestureRecognizer()
      ..onTap = () async {
        final url = linkConfig!.backend.getSearchUrl(text);
        final uri = Uri.parse(url);
        if (!await launchUrl(uri)) {
          throw Exception('Could not launch $uri');
        }
      };
  }

  TextSpan toTextSpan(BuildContext context) {
    final bool containsKanji = hasKanji(text);
    final bool inKanjiBank = kanjiBank[text] != null ? true : false;
    final bool isVocabulary =
        inKanjiBank && kanjiBank[text]!.type == KanjiType.vocabulary;

    if (containsKanji && inKanjiBank && isVocabulary) {
      return TextSpan(
        text: linkConfig?.addSpace == true ? "$text " : text,
        style: const TextStyle(color: Colors.purple),
        recognizer: linkConfig?.linkKnown == true
            ? _getTapRecognizer(text)
            : null,
      );
    } else if (containsKanji && inKanjiBank) {
      return TextSpan(
        text: linkConfig?.addSpace == true ? "$text " : text,

        style: const TextStyle(color: Colors.pink),
        recognizer: linkConfig?.linkUnknown == true
            ? _getTapRecognizer(text)
            : null,
      );
    } else if (containsKanji) {
      final splitText = text.split('');
      return TextSpan(
        children: splitText.indexed.map((rec) {
          final (i, char) = rec;
          final entry = kanjiBank[char];
          final color = (entry != null)
              ? (entry.type == KanjiType.vocabulary
                    ? Colors.purple
                    : Colors.pink)
              : Theme.of(context).colorScheme.onSurface;

          return TextSpan(
            text: i < splitText.length - 1 ? char : "$char ",
            style: TextStyle(color: color),
            recognizer:
                (entry != null && linkConfig?.linkKnown == true) ||
                    (linkConfig?.linkUnknown == true)
                ? _getTapRecognizer(text)
                : null,
          );
        }).toList(),
      );
    }
    return TextSpan(
      text: linkConfig?.addSpace == true ? "$text " : text,
      style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
      recognizer: (linkConfig?.linkUnknown == true)
          ? _getTapRecognizer(text)
          : null,
    );
  }
}
