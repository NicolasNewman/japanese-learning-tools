import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/models/enums.dart';
import 'package:webview_flutter/webview_flutter.dart';

class JishoFrame extends ConsumerStatefulWidget {
  final String searchTerm;
  final void Function() clearSelection;

  const JishoFrame({
    super.key,
    required this.searchTerm,
    required this.clearSelection,
  });

  @override
  ConsumerState<JishoFrame> createState() => _JishoFrameState();
}

class _JishoFrameState extends ConsumerState<JishoFrame> {
  late final WebViewController controller;
  late final WebViewCookieManager cookieManager;
  bool isLoading = true;
  String? _loadedUrl;

  @override
  void initState() {
    super.initState();

    cookieManager = WebViewCookieManager();

    controller = WebViewController()
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setUserAgent(
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
      )
      ..setNavigationDelegate(
        NavigationDelegate(
          onPageStarted: (url) {
            if (mounted) setState(() => isLoading = true);
          },
          onPageFinished: (url) {
            if (mounted) setState(() => isLoading = false);
          },
          onWebResourceError: (error) {
            print('WebView error: ${error.description}');
          },
        ),
      )
      ..enableZoom(false);
  }

  Future<void> _setCookies(
    String domain,
    DictionaryBackendType backend,
    ThemeMode themeMode,
  ) async {
    // Set the theme cookie
    if (backend == DictionaryBackendType.takoboto) {
      final themeValue = themeMode == ThemeMode.light ? 'light' : 'dark';
      await cookieManager.setCookie(
        WebViewCookie(
          name: 'theme',
          value: themeValue,
          domain: domain,
          path: '/',
        ),
      );
    }
  }

  Future<void> _loadUrl(
    String url,
    DictionaryBackendType backend,
    ThemeMode themeMode,
  ) async {
    if (_loadedUrl != url) {
      _loadedUrl = url;

      final uri = Uri.parse(url);
      final domain = uri.host;

      await _setCookies(domain, backend, themeMode);

      controller.loadRequest(Uri.parse(url));
    }
  }

  @override
  Widget build(BuildContext context) {
    final dictionaryBackend = ref.watch(dictionaryBackendProvider);
    final themeMode = ref.watch(themeModeSettingProvider);

    if (themeMode.isLoading || dictionaryBackend.isLoading) {
      return Scaffold(
        appBar: AppBar(title: Text('Loading...')),
        body: Center(child: CircularProgressIndicator()),
      );
    }

    if (themeMode.hasError) {
      return Scaffold(
        appBar: AppBar(title: Text('Error')),
        body: Center(child: Text('Error loading theme: ${themeMode.error}')),
      );
    }
    if (dictionaryBackend.hasError) {
      return Scaffold(
        appBar: AppBar(title: Text('Error')),
        body: Center(
          child: Text(
            'Error loading dictionary backend: ${dictionaryBackend.error}',
          ),
        ),
      );
    }

    final backend = dictionaryBackend.value!;
    final theme = themeMode.value!;

    final url = backend.getSearchUrl(widget.searchTerm);
    _loadUrl(url, backend, theme);

    return Scaffold(
      appBar: AppBar(
        title: Text(backend.toString()),
        actions: [
          if (isLoading)
            Center(
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16.0),
                child: SizedBox(
                  width: 20,
                  height: 20,
                  child: CircularProgressIndicator(strokeWidth: 2),
                ),
              ),
            ),
          IconButton(
            icon: const Icon(Icons.close),
            onPressed: widget.clearSelection,
          ),
        ],
      ),
      body: WebViewWidget(controller: controller),
    );
  }
}
