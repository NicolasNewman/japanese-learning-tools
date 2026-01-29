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

String _$sudachiEndpointHash() => r'8428356db3ef68564cdb7cf62ce4d7150e289b67';

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

String _$waniKaniAPIKeyHash() => r'5222780d6f89c33569d8f56a04d784605db37474';

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
    r'bf26c46ee596f09cbb5d041c3e87b52db6030f3e';

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

String _$kanjiBankHash() => r'3ab9ee37be65151ed58acae460880ac9433c4ee7';

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
