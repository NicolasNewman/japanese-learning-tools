// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'persistence.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(sharedPrefs)
final sharedPrefsProvider = SharedPrefsProvider._();

final class SharedPrefsProvider
    extends
        $FunctionalProvider<
          AsyncValue<SharedPreferences>,
          SharedPreferences,
          FutureOr<SharedPreferences>
        >
    with
        $FutureModifier<SharedPreferences>,
        $FutureProvider<SharedPreferences> {
  SharedPrefsProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'sharedPrefsProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$sharedPrefsHash();

  @$internal
  @override
  $FutureProviderElement<SharedPreferences> $createElement(
    $ProviderPointer pointer,
  ) => $FutureProviderElement(pointer);

  @override
  FutureOr<SharedPreferences> create(Ref ref) {
    return sharedPrefs(ref);
  }
}

String _$sharedPrefsHash() => r'707e23210559b373c587233dd97b87ac9c3b9f57';

@ProviderFor(SudachiEndpoint)
final sudachiEndpointProvider = SudachiEndpointProvider._();

final class SudachiEndpointProvider
    extends $AsyncNotifierProvider<SudachiEndpoint, String> {
  SudachiEndpointProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'sudachiEndpointProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$sudachiEndpointHash();

  @$internal
  @override
  SudachiEndpoint create() => SudachiEndpoint();
}

String _$sudachiEndpointHash() => r'e74672923152421fb8e924ac84007cebeae0a2cf';

abstract class _$SudachiEndpoint extends $AsyncNotifier<String> {
  FutureOr<String> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<AsyncValue<String>, String>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<AsyncValue<String>, String>,
              AsyncValue<String>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}

@ProviderFor(WaniKaniAPIKey)
final waniKaniAPIKeyProvider = WaniKaniAPIKeyProvider._();

final class WaniKaniAPIKeyProvider
    extends $AsyncNotifierProvider<WaniKaniAPIKey, String> {
  WaniKaniAPIKeyProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'waniKaniAPIKeyProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$waniKaniAPIKeyHash();

  @$internal
  @override
  WaniKaniAPIKey create() => WaniKaniAPIKey();
}

String _$waniKaniAPIKeyHash() => r'49d505dd873e5eb3c3fb5a4ebf84e6f9d230c5be';

abstract class _$WaniKaniAPIKey extends $AsyncNotifier<String> {
  FutureOr<String> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<AsyncValue<String>, String>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<AsyncValue<String>, String>,
              AsyncValue<String>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}

@ProviderFor(WaniKaniLastUpdated)
final waniKaniLastUpdatedProvider = WaniKaniLastUpdatedProvider._();

final class WaniKaniLastUpdatedProvider
    extends $AsyncNotifierProvider<WaniKaniLastUpdated, String> {
  WaniKaniLastUpdatedProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'waniKaniLastUpdatedProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$waniKaniLastUpdatedHash();

  @$internal
  @override
  WaniKaniLastUpdated create() => WaniKaniLastUpdated();
}

String _$waniKaniLastUpdatedHash() =>
    r'4a08f32a0280825ca2fe1e22a2a9ab07a46e888d';

abstract class _$WaniKaniLastUpdated extends $AsyncNotifier<String> {
  FutureOr<String> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<AsyncValue<String>, String>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<AsyncValue<String>, String>,
              AsyncValue<String>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}

@ProviderFor(KanjiBank)
final kanjiBankProvider = KanjiBankProvider._();

final class KanjiBankProvider
    extends $AsyncNotifierProvider<KanjiBank, KanjiBankData<dynamic>> {
  KanjiBankProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'kanjiBankProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$kanjiBankHash();

  @$internal
  @override
  KanjiBank create() => KanjiBank();
}

String _$kanjiBankHash() => r'd8ee88f6f67270124c6d1ffeaf826715c2adffc1';

abstract class _$KanjiBank extends $AsyncNotifier<KanjiBankData<dynamic>> {
  FutureOr<KanjiBankData<dynamic>> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref
            as $Ref<AsyncValue<KanjiBankData<dynamic>>, KanjiBankData<dynamic>>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<
                AsyncValue<KanjiBankData<dynamic>>,
                KanjiBankData<dynamic>
              >,
              AsyncValue<KanjiBankData<dynamic>>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}

@ProviderFor(ThemeModeSetting)
final themeModeSettingProvider = ThemeModeSettingProvider._();

final class ThemeModeSettingProvider
    extends $AsyncNotifierProvider<ThemeModeSetting, ThemeMode> {
  ThemeModeSettingProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'themeModeSettingProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$themeModeSettingHash();

  @$internal
  @override
  ThemeModeSetting create() => ThemeModeSetting();
}

String _$themeModeSettingHash() => r'8a70aa30cc7a6fa576ef7af7014aaf823edc68f5';

abstract class _$ThemeModeSetting extends $AsyncNotifier<ThemeMode> {
  FutureOr<ThemeMode> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<AsyncValue<ThemeMode>, ThemeMode>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<AsyncValue<ThemeMode>, ThemeMode>,
              AsyncValue<ThemeMode>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}

@ProviderFor(DictionaryBackend)
final dictionaryBackendProvider = DictionaryBackendProvider._();

final class DictionaryBackendProvider
    extends $AsyncNotifierProvider<DictionaryBackend, DictionaryBackendType> {
  DictionaryBackendProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'dictionaryBackendProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$dictionaryBackendHash();

  @$internal
  @override
  DictionaryBackend create() => DictionaryBackend();
}

String _$dictionaryBackendHash() => r'c06a116f98ca4fb99cce019551a7944c36d7742a';

abstract class _$DictionaryBackend
    extends $AsyncNotifier<DictionaryBackendType> {
  FutureOr<DictionaryBackendType> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref
            as $Ref<AsyncValue<DictionaryBackendType>, DictionaryBackendType>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<
                AsyncValue<DictionaryBackendType>,
                DictionaryBackendType
              >,
              AsyncValue<DictionaryBackendType>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
