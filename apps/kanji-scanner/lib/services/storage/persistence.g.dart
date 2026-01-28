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

String _$sudachiEndpointHash() => r'f748a54b39b0dcf6be20d9a9c0bb4dd0b9e32ba6';

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

String _$waniKaniAPIKeyHash() => r'0c2e1c226691079509ada8cb7cfbcf73f3e0948b';

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

@ProviderFor(KanjiBank)
final kanjiBankProvider = KanjiBankFamily._();

final class KanjiBankProvider<T>
    extends $AsyncNotifierProvider<KanjiBank<T>, KanjiBankData<T>> {
  KanjiBankProvider._({required KanjiBankFamily super.from})
    : super(
        argument: null,
        retry: null,
        name: r'kanjiBankProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$kanjiBankHash();

  @override
  String toString() {
    return r'kanjiBankProvider'
        '<${T}>'
        '()';
  }

  @$internal
  @override
  KanjiBank<T> create() => KanjiBank<T>();

  $R _captureGenerics<$R>($R Function<T>() cb) {
    return cb<T>();
  }

  @override
  bool operator ==(Object other) {
    return other is KanjiBankProvider &&
        other.runtimeType == runtimeType &&
        other.argument == argument;
  }

  @override
  int get hashCode {
    return Object.hash(runtimeType, argument);
  }
}

String _$kanjiBankHash() => r'aba87f2d1e21f988a3d998eec0f3a3fe76f18893';

final class KanjiBankFamily extends $Family {
  KanjiBankFamily._()
    : super(
        retry: null,
        name: r'kanjiBankProvider',
        dependencies: null,
        $allTransitiveDependencies: null,
        isAutoDispose: true,
      );

  KanjiBankProvider<T> call<T>() => KanjiBankProvider<T>._(from: this);

  @override
  String toString() => r'kanjiBankProvider';

  /// {@macro riverpod.override_with}
  Override overrideWith(KanjiBank<T> Function<T>() create) => $FamilyOverride(
    from: this,
    createElement: (pointer) {
      final provider = pointer.origin as KanjiBankProvider;
      return provider._captureGenerics(<T>() {
        provider as KanjiBankProvider<T>;
        return provider.$view(create: create<T>).$createElement(pointer);
      });
    },
  );

  /// {@macro riverpod.override_with_build}
  Override overrideWithBuild(
    FutureOr<KanjiBankData<T>> Function<T>(Ref ref, KanjiBank<T> notifier)
    build,
  ) => $FamilyOverride(
    from: this,
    createElement: (pointer) {
      final provider = pointer.origin as KanjiBankProvider;
      return provider._captureGenerics(<T>() {
        provider as KanjiBankProvider<T>;
        return provider
            .$view(runNotifierBuildOverride: build<T>)
            .$createElement(pointer);
      });
    },
  );
}

abstract class _$KanjiBank<T> extends $AsyncNotifier<KanjiBankData<T>> {
  FutureOr<KanjiBankData<T>> build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref as $Ref<AsyncValue<KanjiBankData<T>>, KanjiBankData<T>>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<AsyncValue<KanjiBankData<T>>, KanjiBankData<T>>,
              AsyncValue<KanjiBankData<T>>,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
