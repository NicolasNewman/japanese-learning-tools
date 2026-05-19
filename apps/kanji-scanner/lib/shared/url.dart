import 'package:flutter/material.dart';
import 'package:kanji_scanner/shared/models/enums.dart';
import 'package:webview_flutter/webview_flutter.dart';

Future<void> setCookiesForDomain(
  String domain,
  DictionaryBackendType backend,
  ThemeMode themeMode,
  WebViewCookieManager cookieManager,
) async {
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
