import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter/material.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';

class HomeView extends ConsumerWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final kanjiBank = ref.watch(kanjiBankProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('Home')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            kanjiBank.when(
              data: (data) =>
                  Text('Kanji Bank loaded with ${data.keys.length} items.'),
              loading: () => const CircularProgressIndicator(),
              error: (error, stack) => Text('Error loading Kanji Bank: $error'),
            ),
          ],
        ),
      ),
    );
  }
}
