class EnvConfig {
  static const String sudachiEndpoint = String.fromEnvironment(
    'SUDACHI_ENDPOINT',
  );
  static const String waniKaniApiKey = String.fromEnvironment(
    'WANIKANI_API_KEY',
  );
  static const String? rawSentence = bool.hasEnvironment("RAW_SENTENCE")
      ? String.fromEnvironment('RAW_SENTENCE')
      : null;
}
