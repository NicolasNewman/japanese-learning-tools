enum Level {
  apprentice,
  guru,
  master,
  enlightened,
  burned;

  static Level fromSrsStage(int stage) {
    if (stage <= 4) {
      return Level.apprentice;
    } else if (stage <= 6) {
      return Level.guru;
    } else if (stage <= 7) {
      return Level.master;
    } else if (stage <= 8) {
      return Level.enlightened;
    } else {
      return Level.burned;
    }
  }
}

enum KanjiSource {
  wanikani,
  anki;

  String get registryKey {
    switch (this) {
      case KanjiSource.wanikani:
        return 'wanikani';
      case KanjiSource.anki:
        return 'anki';
    }
  }

  static KanjiSource fromRegistryKey(String key) {
    return KanjiSource.values.firstWhere(
      (source) => source.registryKey == key,
      orElse: () => KanjiSource.wanikani,
    );
  }
}

class KanjiType {
  static const String kanji = 'kanji';
  static const String vocabulary = 'vocabulary';
}

class KanjiBankEntry<T> {
  final Level level;
  final String type; // 'kanji' or 'vocabulary'
  final KanjiSource source;
  final String meaning;
  final T metadata;

  KanjiBankEntry({
    required this.level,
    required this.type,
    required this.source,
    required this.meaning,
    required this.metadata,
  });

  Map<String, dynamic> toJson() {
    dynamic metadataJson = metadata;
    try {
      metadataJson = (metadata as dynamic).toJson();
    } catch (_) {
      // If toJson doesn't exist, keep as-is
    }
    return {
      'level': level.index,
      'type': type,
      'source': source.index,
      'meaning': meaning,
      'metadata': metadataJson,
    };
  }

  factory KanjiBankEntry.fromJson(Map<String, dynamic> json) {
    final source = KanjiSource.values[json['source']];
    final deserializer = MetadataRegistry.getOrThrow(source.registryKey);

    return KanjiBankEntry(
      level: Level.values[json['level']],
      type: json['type'],
      source: source,
      meaning: json['meaning'],
      metadata: deserializer(json['metadata']),
    );
  }
}

typedef KanjiBankData<T> = Map<String, KanjiBankEntry<T>>;

Map<String, dynamic> kanjiBankDataToJson<T>(KanjiBankData<T> data) {
  return data.map((key, value) => MapEntry(key, value.toJson()));
}

KanjiBankData kanjiBankDataFromJson(Map<String, dynamic> json) {
  return json.map(
    (key, value) =>
        MapEntry(key, KanjiBankEntry.fromJson(value as Map<String, dynamic>)),
  );
}

typedef MetadataDeserializer<T> = T Function(Map<String, dynamic>);

class MetadataRegistry {
  static final Map<String, MetadataDeserializer> _deserializers = {};

  static void register<T>(
    String typeKey,
    MetadataDeserializer<T> deserializer,
  ) {
    _deserializers[typeKey] = deserializer;
  }

  static MetadataDeserializer? get(String typeKey) {
    return _deserializers[typeKey];
  }

  static MetadataDeserializer getOrThrow(String typeKey) {
    final deserializer = get(typeKey);
    if (deserializer == null) {
      throw Exception('No deserializer registered for type: $typeKey');
    }
    return deserializer;
  }
}
