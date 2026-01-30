import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/widgets/text_field.dart';

class GeneralTab extends ConsumerStatefulWidget {
  const GeneralTab({super.key, required this.getController});

  final TextEditingController Function(String) getController;

  @override
  ConsumerState<GeneralTab> createState() => _GeneralTabState();
}

class _GeneralTabState extends ConsumerState<GeneralTab> {
  @override
  Widget build(BuildContext context) {
    final endpoint = ref.watch(sudachiEndpointProvider);

    return Center(
      child: SizedBox(
        width: 250,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            buildTextFieldWidget(
              endpoint,
              widget.getController('endpoint'),
              'Sudachi Endpoint',
              (newValue) {
                ref
                    .read(sudachiEndpointProvider.notifier)
                    .setEndpoint(newValue);
              },
            ),
            // SizedBox(height: 16),
          ],
        ),
      ),
    );
  }
}
