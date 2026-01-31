// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'state.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(RawSentence)
final rawSentenceProvider = RawSentenceProvider._();

final class RawSentenceProvider
    extends $NotifierProvider<RawSentence, String?> {
  RawSentenceProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'rawSentenceProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$rawSentenceHash();

  @$internal
  @override
  RawSentence create() => RawSentence();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(String? value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<String?>(value),
    );
  }
}

String _$rawSentenceHash() => r'af50ac5097b3cedd4769586f17a33732e1f70657';

abstract class _$RawSentence extends $Notifier<String?> {
  String? build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<String?, String?>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<String?, String?>,
              String?,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}

@ProviderFor(parsedSentence)
final parsedSentenceProvider = ParsedSentenceProvider._();

final class ParsedSentenceProvider
    extends
        $FunctionalProvider<
          AsyncValue<List<TokenInfo>?>,
          List<TokenInfo>?,
          FutureOr<List<TokenInfo>?>
        >
    with $FutureModifier<List<TokenInfo>?>, $FutureProvider<List<TokenInfo>?> {
  ParsedSentenceProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'parsedSentenceProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$parsedSentenceHash();

  @$internal
  @override
  $FutureProviderElement<List<TokenInfo>?> $createElement(
    $ProviderPointer pointer,
  ) => $FutureProviderElement(pointer);

  @override
  FutureOr<List<TokenInfo>?> create(Ref ref) {
    return parsedSentence(ref);
  }
}

String _$parsedSentenceHash() => r'cb71ab35f3b552dcac7b370cdc16738ce88df2f3';
