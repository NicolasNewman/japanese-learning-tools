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
          AsyncValue<(SudachiResponse, KanjiBankData<dynamic>)>,
          (SudachiResponse, KanjiBankData<dynamic>),
          FutureOr<(SudachiResponse, KanjiBankData<dynamic>)>
        >
    with
        $FutureModifier<(SudachiResponse, KanjiBankData<dynamic>)>,
        $FutureProvider<(SudachiResponse, KanjiBankData<dynamic>)> {
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
  $FutureProviderElement<(SudachiResponse, KanjiBankData<dynamic>)>
  $createElement($ProviderPointer pointer) => $FutureProviderElement(pointer);

  @override
  FutureOr<(SudachiResponse, KanjiBankData<dynamic>)> create(Ref ref) {
    return analyzedData(ref);
  }
}

String _$analyzedDataHash() => r'9994520c0e2293657393f26b31d38905ad4eba92';
