import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:kanji_scanner/shared/models/sudachi.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'state.g.dart';

@riverpod
class RawSentence extends _$RawSentence {
  @override
  String? build() {
    // return "は美人の代名詞とされる。は美人の代名詞とされる。";
    return null;
  }

  void update(String? newSentence) {
    state = newSentence;
  }
}

@riverpod
Future<SudachiResponse> parsedSentence(Ref ref) async {
  final sentence = ref.watch(rawSentenceProvider);
  if (sentence == null) {
    return SudachiResponse(response: null);
  }
  final response = await http.post(
    Uri.parse('http://10.0.0.249:8020/api/sudachi'),
    headers: {'Content-Type': 'application/json'},
    body: jsonEncode({'input': sentence, "wakati": true}),
  );

  if (response.statusCode != 200) {
    throw Exception('Failed to analyze text');
  }
  return SudachiResponse.fromJson(jsonDecode(response.body));
}
