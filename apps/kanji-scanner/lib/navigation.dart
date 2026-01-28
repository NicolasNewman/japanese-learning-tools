
import 'package:flutter/material.dart';
import 'package:kanji_scanner/features/analyzer/analyze_view.dart';
import 'package:kanji_scanner/features/scanner/scanner_view.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
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
    final ThemeData theme = Theme.of(context);
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
        ],
      ),
      body: <Widget>[
        /// Home page
        Card(
          shadowColor: Colors.transparent,
          margin: const EdgeInsets.all(8.0),
          child: SizedBox.expand(
            child: Center(
              child: Text("Home", style: theme.textTheme.titleLarge),
            ),
          ),
        ),

        /// Scan page
        TextRecognizerView(),

        /// Analyze page
        AnalyzerView(),
      ][currentPageIndex],
    );
  }
}