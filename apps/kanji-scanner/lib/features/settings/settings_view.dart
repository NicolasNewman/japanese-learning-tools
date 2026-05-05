import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/features/settings/general_tab.dart';
import 'package:kanji_scanner/features/settings/wanikani_tab.dart';
import 'package:kanji_scanner/features/settings/anki_tab.dart';

class SettingsView extends ConsumerStatefulWidget {
  const SettingsView({super.key});

  @override
  ConsumerState<SettingsView> createState() => _SettingsViewState();
}

class _SettingsViewState extends ConsumerState<SettingsView> {
  final Map<String, TextEditingController> _controllers = {};

  TextEditingController _getController(String key) {
    return _controllers.putIfAbsent(key, () => TextEditingController());
  }

  @override
  void dispose() {
    for (var controller in _controllers.values) {
      controller.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 3,
      initialIndex: 0,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Settings'),
          bottom: const TabBar(
            tabs: [
              Tab(text: 'General'),
              Tab(text: 'WaniKani'),
              Tab(text: 'Anki'),
            ],
          ),
        ),
        body: TabBarView(
          children: <Widget>[
            GeneralTab(),
            WanikaniTab(getController: _getController),
            AnkiTab(getController: _getController),
          ],
        ),
      ),
    );
  }
}
