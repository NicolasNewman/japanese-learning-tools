import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/models/enums.dart';

class GeneralTab extends ConsumerWidget {
  const GeneralTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final themeModeAsync = ref.watch(themeModeSettingProvider);
    final dictionaryBackendAsync = ref.watch(dictionaryBackendProvider);

    if (themeModeAsync.isLoading || dictionaryBackendAsync.isLoading) {
      return const Center(child: CircularProgressIndicator());
    }

    if (themeModeAsync.hasError) {
      return Center(
        child: Text('Error loading theme: ${themeModeAsync.error}'),
      );
    }
    if (dictionaryBackendAsync.hasError) {
      return Center(
        child: Text('Error loading backend: ${dictionaryBackendAsync.error}'),
      );
    }

    final themeMode = themeModeAsync.value!;
    final dictionaryBackend = dictionaryBackendAsync.value!;

    return SizedBox(
      width: 250,
      child: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // buildTextFieldWidget(
            //   endpoint,
            //   widget.getController('endpoint'),
            //   'Sudachi Endpoint',
            //   (newValue) {
            //     ref
            //         .read(sudachiEndpointProvider.notifier)
            //         .setEndpoint(newValue);
            //   },
            // ),
            ListTile(title: Text('Dictionary Backend')),
            RadioGroup<DictionaryBackendType>(
              groupValue: dictionaryBackend,
              onChanged: (v) => {
                ref
                    .read(dictionaryBackendProvider.notifier)
                    .setDictionaryBackend(v as DictionaryBackendType),
              },
              child: Column(
                children: DictionaryBackendType.values.map((backend) {
                  return RadioListTile<DictionaryBackendType>(
                    title: Text(backend.toString().split('.').last),
                    value: backend,
                  );
                }).toList(),
              ),
            ),
            SizedBox(height: 16),
            ListTile(title: Text('Theme Mode')),
            SegmentedButton(
              segments: [
                ButtonSegment(
                  value: ThemeMode.light,
                  label: Text('Light'),
                  icon: Icon(Icons.light_mode),
                ),
                ButtonSegment(
                  value: ThemeMode.dark,
                  label: Text('Dark'),
                  icon: Icon(Icons.dark_mode),
                ),
                ButtonSegment(
                  value: ThemeMode.system,
                  label: Text('System'),
                  icon: Icon(Icons.settings),
                ),
              ],
              selected: {themeMode},
              onSelectionChanged: (newSelection) {
                final newMode = newSelection.first;
                ref
                    .read(themeModeSettingProvider.notifier)
                    .setThemeMode(newMode);
              },
              multiSelectionEnabled: false,
            ),
          ],
        ),
      ),
    );
  }
}
