import 'dart:convert';

import 'package:kanji_scanner/core/env.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';

part 'persistence.g.dart';

@riverpod
Future<SharedPreferences> sharedPrefs(Ref ref) async {
  return await SharedPreferences.getInstance();
}

@riverpod
class SudachiEndpoint extends _$SudachiEndpoint {
  SharedPreferences? prefs;

  @override
  Future<String> build() async {
    prefs = await (ref.watch(sharedPrefsProvider.future));
    final endpoint = prefs!.getString('sudachi_endpoint');
    if (endpoint != null && endpoint.isNotEmpty) {
      return endpoint;
    }
    return EnvConfig.sudachiEndpoint;
  }

  Future<void> setEndpoint(String newEndpoint) async {
    await prefs!.setString('sudachi_endpoint', newEndpoint);
    state = AsyncValue.data(newEndpoint);
  }
}

@riverpod
class WaniKaniAPIKey extends _$WaniKaniAPIKey {
  SharedPreferences? prefs;

  @override
  Future<String> build() async {
    prefs = await (ref.watch(sharedPrefsProvider.future));
    final apiKey = prefs!.getString('wanikani_api_key');
    if (apiKey != null && apiKey.isNotEmpty) {
      return apiKey;
    }
    return EnvConfig.waniKaniApiKey;
  }

  Future<void> setAPIKey(String newAPIKey) async {
    await prefs!.setString('wanikani_api_key', newAPIKey);
    state = AsyncValue.data(newAPIKey);
  }
}

@riverpod
class KanjiBank<T> extends _$KanjiBank<T> {
  SharedPreferences? prefs;

  @override
  Future<KanjiBankData<T>> build() async {
    prefs = await (ref.watch(sharedPrefsProvider.future));
    final jsonString = prefs!.getString('kanji_bank_data');
    if (jsonString != null && jsonString.isNotEmpty) {
      return jsonDecode(jsonString);
    }
    return {};
  }

  Future<void> setKanjiBankData(KanjiBankData<T> newData) async {
    final jsonString = jsonEncode(newData);
    await prefs!.setString('kanji_bank_data', jsonString);
    state = AsyncValue.data(newData);
  }
}
