import 'dart:io';

import 'package:flutter/services.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';

Future<String> getAssetPath(String asset) async {
  final path = await getLocalPath(asset);
  await Directory(dirname(path)).create(recursive: true);
  final file = File(path);
  if (!await file.exists()) {
    final byteData = await rootBundle.load(asset);
    await file.writeAsBytes(
      byteData.buffer.asUint8List(
        byteData.offsetInBytes,
        byteData.lengthInBytes,
      ),
    );
  }
  return file.path;
}

Future<String> getLocalPath(String path) async {
  return '${(await getApplicationSupportDirectory()).path}/$path';
}

bool hasKanji(String text) {
  // CJK Unified Ideographs + Extensions + Compatibility Ideographs
  // Ranges (inclusive):
  // - U+3400–U+4DBF   CJK Unified Ideographs Extension A
  // - U+4E00–U+9FFF   CJK Unified Ideographs (main Kanji block)
  // - U+F900–U+FAFF   CJK Compatibility Ideographs
  // - U+20000–U+2A6DF CJK Unified Ideographs Extension B
  // - U+2A700–U+2B73F Extension C
  // - U+2B740–U+2B81F Extension D
  // - U+2B820–U+2CEAF Extension E
  // - U+2CEB0–U+2EBEF Extension F
  // - U+30000–U+3134F Extension G
  // - U+2F800–U+2FA1F CJK Compatibility Ideographs Supplement
  final kanji = RegExp(
    r'[\u3400-\u4DBF\u4E00-\u9FFF\uF900-\uFAFF]'
    r'|[\u{20000}-\u{2A6DF}]'
    r'|[\u{2A700}-\u{2B73F}]'
    r'|[\u{2B740}-\u{2B81F}]'
    r'|[\u{2B820}-\u{2CEAF}]'
    r'|[\u{2CEB0}-\u{2EBEF}]'
    r'|[\u{30000}-\u{3134F}]'
    r'|[\u{2F800}-\u{2FA1F}]',
    unicode: true,
  );

  return kanji.hasMatch(text);
}
