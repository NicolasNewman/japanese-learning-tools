import 'package:kanji_scanner/core/env.dart';
import 'package:kanji_scanner/src/rust/api/sudachi_api.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'state.g.dart';

@riverpod
class RawSentence extends _$RawSentence {
  @override
  String? build() {
    return EnvConfig.rawSentence;
  }

  void update(String? newSentence) {
    state = newSentence;
  }
}

@riverpod
Future<List<TokenInfo>?> parsedSentence(Ref ref) async {
  final sentence = ref.watch(rawSentenceProvider);
  if (sentence == null) {
    return null;
  }
  try {
    final tokens = await sudachiRs(text: sentence);
    return tokens;
  } catch (e) {
    throw Exception('Failed to analyze text: $e');
  }
}

// @riverpod
// Future<SudachiResponse> parsedSentence(Ref ref) async {
//   final sentence = ref.watch(rawSentenceProvider);
//   if (sentence == null) {
//     return SudachiResponse(response: null);
//   }
//   final endpoint = await ref.read(sudachiEndpointProvider.future);
//   final response = await http.post(
//     Uri.parse(endpoint),
//     headers: {'Content-Type': 'application/json'},
//     body: jsonEncode({'input': sentence, "wakati": true}),
//   );

//   if (response.statusCode != 200) {
//     throw Exception('Failed to analyze text');
//   }
//   return SudachiResponse.fromJson(jsonDecode(response.body));
// }
