import 'package:flutter/material.dart';
import 'package:flutter_widget_from_html_core/flutter_widget_from_html_core.dart';
import 'package:fwfh_webview/fwfh_webview.dart';

class JishoFrame extends StatelessWidget {
  final String searchTerm;
  final void Function() clearSelection;

  const JishoFrame({
    super.key,
    required this.searchTerm,
    required this.clearSelection,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Jisho'),
        actions: [
          IconButton(
            icon: const Icon(Icons.close),
            onPressed: () {
              clearSelection();
            },
          ),
        ],
      ),
      body: SizedBox(
        width: double.infinity,
        height: double.infinity,
        child: HtmlWidget(
          '<iframe src="https://jisho.org/search/$searchTerm"></iframe>',
          factoryBuilder: () => JishoWidgetFactory(),
        ),
      ),
    );
  }
}

class JishoWidgetFactory extends WidgetFactory with WebViewFactory {}
