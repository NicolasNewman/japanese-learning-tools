import 'package:flutter/material.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

Widget buildTextFieldWidget(
  AsyncValue<String> asyncValue,
  TextEditingController controller,
  String label,
  void Function(String) onSave, {
  bool obscureText = false,
}) {
  return asyncValue.when(
    data: (value) {
      if (controller.text != value) {
        controller.text = value;
      }
      return TextField(
        controller: controller,
        obscureText: obscureText,
        decoration: InputDecoration(
          border: OutlineInputBorder(),
          labelText: label,
        ),
        // TODO: must also handle focus loss to save
        onSubmitted: (newValue) {
          onSave(newValue);
        },
      );
    },
    loading: () => CircularProgressIndicator(),
    error: (err, stack) => Text('Error: $err'),
  );
}
