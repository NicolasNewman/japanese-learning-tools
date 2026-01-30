import 'package:flutter/material.dart';
import 'package:kanji_scanner/navigation.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:kanji_scanner/shared/models/kanji/wanikani.dart';

Future<void> main() async {
  MetadataRegistry.register(
    KanjiSource.wanikani.registryKey,
    WaniKaniMetadata.fromJson,
  );
  runApp(const ProviderScope(child: KanjiScanner()));
}

class KanjiScanner extends StatelessWidget {
  const KanjiScanner({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      home: Navigation(),
    );
  }
}
