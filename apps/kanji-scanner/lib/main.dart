// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:kanji_scanner/navigation.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'dart:io';

import 'package:kanji_scanner/src/rust/api/sudachi_api.dart';
import 'package:kanji_scanner/src/rust/frb_generated.dart';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:kanji_scanner/shared/models/kanji/wanikani.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as path;

Future<void> initializeSudachi() async {
  try {
    // Get the app's document directory
    final appDir = await getApplicationDocumentsDirectory();
    final dictPath = path.join(appDir.path, 'system.dic');

    // Check if dictionary already exists
    final dictFile = File(dictPath);
    if (!await dictFile.exists()) {
      print('Copying dictionary file to $dictPath');

      // Copy the dictionary from assets to the app directory
      final byteData = await rootBundle.load('rust/resources/system.dic');
      final buffer = byteData.buffer;
      await dictFile.writeAsBytes(
        buffer.asUint8List(byteData.offsetInBytes, byteData.lengthInBytes),
      );

      print('Dictionary copied successfully');
    } else {
      print('Dictionary already exists at $dictPath');
    }

    // Initialize Sudachi with the dictionary path
    sudachiInitWithDictPath(dictPath: dictPath);
    print('Sudachi initialized successfully');
  } catch (e) {
    print('Failed to initialize Sudachi: $e');
    rethrow;
  }
}

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await RustLib.init();

  await initializeSudachi();

  MetadataRegistry.register(
    KanjiSource.wanikani.registryKey,
    WaniKaniMetadata.fromJson,
  );
  runApp(const ProviderScope(child: KanjiScanner()));
}

class KanjiScanner extends ConsumerWidget {
  const KanjiScanner({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ThemeMode themeMode = ref
        .watch(themeModeSettingProvider)
        .maybeWhen(data: (mode) => mode, orElse: () => ThemeMode.system);
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.deepPurple,
          brightness: Brightness.light,
        ),
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.deepPurple,
          brightness: Brightness.dark,
        ),
      ),
      themeMode: themeMode,
      home: Navigation(),
    );
  }
}
