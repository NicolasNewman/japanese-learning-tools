import 'package:flutter/material.dart';
import 'package:kanji_scanner/features/analyzer/analyze_view.dart';
import 'package:kanji_scanner/features/home/home_view.dart';
import 'package:kanji_scanner/features/scanner/scanner_view.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/features/settings/settings_view.dart';
import 'package:kanji_scanner/shared/providers/state.dart';

class Navigation extends ConsumerStatefulWidget {
  const Navigation({super.key});

  @override
  ConsumerState<Navigation> createState() => _NavigationState();
}

class _NavigationState extends ConsumerState<Navigation> {
  int currentPageIndex = 0;

  @override
  Widget build(BuildContext context) {
    final parsedSentence = ref.watch(parsedSentenceProvider);

    return Scaffold(
      bottomNavigationBar: NavigationBar(
        onDestinationSelected: (int index) {
          setState(() {
            currentPageIndex = index;
          });
        },
        indicatorColor: Colors.amber,
        selectedIndex: currentPageIndex,
        destinations: <Widget>[
          const NavigationDestination(
            selectedIcon: Icon(Icons.home),
            icon: Icon(Icons.home_outlined),
            label: 'Home',
          ),
          const NavigationDestination(
            icon: Badge(child: Icon(Icons.camera)),
            label: 'Scan',
          ),
          NavigationDestination(
            icon: const Badge(child: Icon(Icons.language)),
            label: 'Analyze',
            enabled: parsedSentence.when(
              data: (data) => data.response != null,
              loading: () => false,
              error: (error, stack) => false,
            ),
          ),
          const NavigationDestination(
            selectedIcon: Icon(Icons.settings),
            icon: Icon(Icons.settings_outlined),
            label: 'Settings',
          ),
        ],
      ),
      body: <Widget>[
        /// Home page
        HomeView(),

        /// Scan page
        TextRecognizerView(),

        /// Analyze page
        AnalyzerView(),

        /// Settings page
        SettingsView(),
      ][currentPageIndex],
    );
  }
}
