// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'analyze_view.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(analyzedData)
final analyzedDataProvider = AnalyzedDataProvider._();

final class AnalyzedDataProvider
    extends
        $FunctionalProvider<
          AsyncValue<(List<TokenInfo>?, KanjiBankData<dynamic>)>,
          (List<TokenInfo>?, KanjiBankData<dynamic>),
          FutureOr<(List<TokenInfo>?, KanjiBankData<dynamic>)>
        >
    with
        $FutureModifier<(List<TokenInfo>?, KanjiBankData<dynamic>)>,
        $FutureProvider<(List<TokenInfo>?, KanjiBankData<dynamic>)> {
  AnalyzedDataProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'analyzedDataProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$analyzedDataHash();

  @$internal
  @override
  $FutureProviderElement<(List<TokenInfo>?, KanjiBankData<dynamic>)>
  $createElement($ProviderPointer pointer) => $FutureProviderElement(pointer);

  @override
  FutureOr<(List<TokenInfo>?, KanjiBankData<dynamic>)> create(Ref ref) {
    return analyzedData(ref);
  }
}

String _$analyzedDataHash() => r'6b781bc24900f2702af0c9ea4b342a7491e98739';
