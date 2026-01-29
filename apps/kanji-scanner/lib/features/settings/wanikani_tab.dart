import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:kanji_scanner/services/api/wanikani_service.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/widgets/text_field.dart';

class WanikaniTab extends ConsumerStatefulWidget {
  const WanikaniTab({super.key, required this.getController});

  final TextEditingController Function(String) getController;

  @override
  ConsumerState<WanikaniTab> createState() => _WanikaniTabState();
}

class _WanikaniTabState extends ConsumerState<WanikaniTab> {
  bool _isLoading = false;

  @override
  Widget build(BuildContext context) {
    final apiKey = ref.watch(waniKaniAPIKeyProvider);
    final lastUpdated = ref.watch(waniKaniLastUpdatedProvider);

    return Center(
      child: SizedBox(
        width: 250,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            buildTextFieldWidget(
              apiKey,
              widget.getController('apiKey'),
              'WaniKani API Key',
              // obscureText: true,
              (newValue) {
                ref.read(waniKaniAPIKeyProvider.notifier).setAPIKey(newValue);
              },
            ),
            Row(
              children: [
                FilledButton(
                  onPressed: _isLoading
                      ? null
                      : () async {
                          setState(() {
                            _isLoading = true;
                          });
                          try {
                            var data = await WaniKaniService(
                              await ref.read(waniKaniAPIKeyProvider.future),
                            ).load();

                            await ref.read(kanjiBankProvider.future);

                            ref
                                .read(kanjiBankProvider.notifier)
                                .setKanjiBankData(data);
                            ref
                                .read(waniKaniLastUpdatedProvider.notifier)
                                .setLastUpdated(
                                  DateFormat(
                                    'yyyy-MM-dd HH:mm',
                                  ).format(DateTime.now()),
                                );
                          } catch (e) {
                            // TODO: show error message
                            print('Error updating WaniKani data: $e');
                          } finally {
                            setState(() {
                              _isLoading = false;
                            });
                          }
                        },
                  child: _isLoading
                      ? SizedBox(
                          height: 24,
                          width: 24,
                          child: CircularProgressIndicator(
                            color: Colors.white,
                            strokeWidth: 2.0,
                          ),
                        )
                      : Text("Update"),
                ),
                SizedBox(width: 16),
                Text("Last updated:\n ${lastUpdated.value}"),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
