import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:google_mlkit_text_recognition/google_mlkit_text_recognition.dart';
import 'package:kanji_scanner/services/storage/persistence.dart';
import 'package:kanji_scanner/shared/providers/state.dart';
import 'package:kanji_scanner/shared/widgets/kanji_bank_text.dart';
import 'package:kanji_scanner/src/rust/api/sudachi_api.dart';

import 'detector_view.dart';
import 'painters/text_detector_painter.dart';

class TextRecognizerView extends ConsumerStatefulWidget {
  const TextRecognizerView({super.key});

  @override
  ConsumerState<TextRecognizerView> createState() => _TextRecognizerViewState();
}

class _TextRecognizerViewState extends ConsumerState<TextRecognizerView> {
  final TextRecognizer _textRecognizer = TextRecognizer(
    script: TextRecognitionScript.japanese,
  );
  bool _canProcess = true;
  bool _isBusy = false;
  CustomPaint? _customPaint;
  String? _text;
  List<KanjiBankText>? _tokens;
  var _cameraLensDirection = CameraLensDirection.back;
  TextRecognizerPainter? _currentPainter;

  void _handleTap(Offset position, Size size) {
    if (_currentPainter != null) {
      final tappedBlock = _currentPainter!.getBlockAtPosition(position, size);
      if (tappedBlock != null) {
        ref.read(rawSentenceProvider.notifier).update(tappedBlock.text);
        showDialog(
          context: context,
          builder: (context) => AlertDialog(
            title: Text('Recognized Text'),
            content: Text(tappedBlock.text),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: Text('OK'),
              ),
            ],
          ),
        );
      }
    }
  }

  @override
  void dispose() async {
    _canProcess = false;
    _textRecognizer.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          DetectorView(
            title: 'Text Detector',
            customPaint: _customPaint,
            text: _text,
            tokens: _tokens,
            onImage: _processImage,
            // initialDetectionMode: DetectorViewMode.gallery,
            initialCameraLensDirection: _cameraLensDirection,
            onCameraLensDirectionChanged: (value) =>
                _cameraLensDirection = value,
            onTap: (position, size) => _handleTap(position, size),
          ),
          // Positioned(
          //   top: 30,
          //   left: 100,
          //   right: 100,
          //   child: Row(
          //     children: [
          //       Spacer(),
          //       Container(
          //         decoration: BoxDecoration(
          //           color: Colors.black54,
          //           borderRadius: BorderRadius.circular(10.0),
          //         ),
          //         child: Padding(
          //           padding: const EdgeInsets.all(4.0),
          //           child: _buildDropdown(),
          //         ),
          //       ),
          //       Spacer(),
          //     ],
          //   ),
          // ),
        ],
      ),
    );
  }

  // Widget _buildDropdown() => DropdownButton<TextRecognitionScript>(
  //   value: _script,
  //   icon: const Icon(Icons.arrow_downward),
  //   elevation: 16,
  //   style: const TextStyle(color: Colors.blue),
  //   underline: Container(height: 2, color: Colors.blue),
  //   onChanged: (TextRecognitionScript? script) {
  //     if (script != null) {
  //       setState(() {
  //         _script = script;
  //         _textRecognizer.close();
  //         _textRecognizer = TextRecognizer(script: _script);
  //       });
  //     }
  //   },
  //   items: TextRecognitionScript.values
  //       .map<DropdownMenuItem<TextRecognitionScript>>((script) {
  //         return DropdownMenuItem<TextRecognitionScript>(
  //           value: script,
  //           child: Text(script.name),
  //         );
  //       })
  //       .toList(),
  // );

  Future<void> _processImage(InputImage inputImage) async {
    if (!_canProcess) return;
    if (_isBusy) return;
    _isBusy = true;
    setState(() {
      _text = '';
      _tokens = null;
    });
    final recognizedText = await _textRecognizer.processImage(inputImage);
    final kanjiBank = await ref.read(kanjiBankProvider.future);
    final backend = await ref.read(dictionaryBackendProvider.future);
    _tokens = (await sudachiRs(text: recognizedText.text))
        .map(
          (token) => KanjiBankText(
            text: token.surface,
            kanjiBank: kanjiBank,
            linkConfig: KanjiBankTextLinkConfig(
              backend: backend,
              linkKnown: true,
              linkUnknown: true,
              addSpace: true,
            ),
          ),
        )
        .toList();

    if (inputImage.metadata?.size != null &&
        inputImage.metadata?.rotation != null) {
      final painter = TextRecognizerPainter(
        recognizedText,
        inputImage.metadata!.size,
        inputImage.metadata!.rotation,
        _cameraLensDirection,
      );
      _currentPainter = painter;
      _customPaint = CustomPaint(painter: painter);
    } else {
      _text = 'Recognized text:\n\n${recognizedText.text}';
      _currentPainter = null;
      _customPaint = null;
    }
    _isBusy = false;
    if (mounted) {
      setState(() {});
    }
  }
}
