import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:kanji_scanner/core/env.dart';
import 'package:kanji_scanner/shared/models/enums.dart';
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
    prefs = await (ref.read(sharedPrefsProvider.future));
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
    prefs = await (ref.read(sharedPrefsProvider.future));
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
class WaniKaniLastUpdated extends _$WaniKaniLastUpdated {
  SharedPreferences? prefs;

  @override
  Future<String> build() async {
    prefs = await (ref.read(sharedPrefsProvider.future));
    final lastUpdated = prefs!.getString('wanikani_last_updated');
    if (lastUpdated != null && lastUpdated.isNotEmpty) {
      return lastUpdated;
    }
    return "never";
  }

  Future<void> setLastUpdated(String newLastUpdated) async {
    await prefs!.setString('wanikani_last_updated', newLastUpdated);
    state = AsyncValue.data(newLastUpdated);
  }
}

@riverpod
class KanjiBank extends _$KanjiBank {
  SharedPreferences? prefs;

  @override
  Future<KanjiBankData> build() async {
    prefs = await (ref.read(sharedPrefsProvider.future));
    final jsonString = prefs!.getString('kanji_bank_data');

    if (jsonString != null && jsonString.isNotEmpty) {
      final decoded = jsonDecode(jsonString) as Map<String, dynamic>;
      return kanjiBankDataFromJson(decoded);
    }
    return {};
  }

  Future<void> setKanjiBankData(KanjiBankData newData) async {
    final jsonData = kanjiBankDataToJson(newData);
    final jsonString = jsonEncode(jsonData);
    await prefs!.setString('kanji_bank_data', jsonString);
    state = AsyncValue.data(newData);
  }
}

@riverpod
class ThemeModeSetting extends _$ThemeModeSetting {
  SharedPreferences? prefs;

  @override
  Future<ThemeMode> build() async {
    prefs = await (ref.read(sharedPrefsProvider.future));
    final themeModeString = prefs!.getString('theme_mode_setting');
    if (themeModeString != null && themeModeString.isNotEmpty) {
      return ThemeMode.values.firstWhere(
        (mode) => mode.toString() == themeModeString,
        orElse: () => ThemeMode.system,
      );
    }
    return ThemeMode.system;
  }

  Future<void> setThemeMode(ThemeMode themeMode) async {
    await prefs!.setString('theme_mode_setting', themeMode.toString());
    state = AsyncValue.data(themeMode);
  }
}

@riverpod
class DictionaryBackend extends _$DictionaryBackend {
  SharedPreferences? prefs;

  @override
  Future<DictionaryBackendType> build() async {
    prefs = await (ref.read(sharedPrefsProvider.future));
    final backendString = prefs!.getString('dictionary_backend');
    if (backendString != null && backendString.isNotEmpty) {
      return DictionaryBackendType.values.firstWhere(
        (backend) => backend.toString() == backendString,
        orElse: () => DictionaryBackendType.jisho,
      );
    }
    return DictionaryBackendType.jisho;
  }

  Future<void> setDictionaryBackend(DictionaryBackendType backend) async {
    await prefs!.setString('dictionary_backend', backend.toString());
    state = AsyncValue.data(backend);
  }
}
