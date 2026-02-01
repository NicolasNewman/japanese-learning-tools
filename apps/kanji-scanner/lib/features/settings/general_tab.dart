import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';

class GeneralTab extends ConsumerWidget {
  const GeneralTab({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final endpoint = ref.watch(sudachiEndpointProvider);
    final themeMode = ref.watch(themeModeSettingProvider);

    return themeMode.when(
      data: (mode) => SizedBox(
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
              SizedBox(height: 16),
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
                selected: {mode},
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
      ),
      loading: () => const CircularProgressIndicator(),
      error: (e, st) => Text('Error loading settings: $e'),
    );
  }
}
