import 'package:flutter/material.dart';
import 'package:kanji_scanner/shared/models/kanji/kanji_bank.dart';
import 'package:kanji_scanner/shared/models/sudachi.dart';
import 'package:kanji_scanner/shared/widgets/kanji_bank_text.dart';

class ListViewWidget extends StatelessWidget {
  final SudachiResponse parsedSentence;
  final KanjiBankData kanjiBank;
  final void Function(String term) triggerJisho;
  final void Function(String term) triggerAnki;

  const ListViewWidget({
    super.key,
    required this.parsedSentence,
    required this.kanjiBank,
    required this.triggerJisho,
    required this.triggerAnki,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(
          parsedSentence.response?.join(" ") ?? "",
          style: Theme.of(context).textTheme.titleLarge,
        ),
      ),
      body: Scrollbar(
        child: ListView.separated(
          itemCount: parsedSentence.response?.length ?? 0,
          separatorBuilder: (context, index) => Divider(),
          itemBuilder: (context, index) {
            final item = parsedSentence.response![index];
            return Dismissible(
              key: Key(item.toString() + index.toString()),
              background: Container(
                color: Colors.green,
                alignment: Alignment.centerLeft,
                padding: EdgeInsets.only(left: 20),
                child: Row(
                  children: [
                    Icon(Icons.check, color: Colors.white),
                    SizedBox(width: 8),
                    Text('Jisho', style: TextStyle(color: Colors.white)),
                  ],
                ),
              ),
              secondaryBackground: Container(
                color: Colors.blue,
                alignment: Alignment.centerRight,
                padding: EdgeInsets.only(right: 20),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    Text('Anki', style: TextStyle(color: Colors.white)),
                    SizedBox(width: 8),
                    Icon(Icons.edit, color: Colors.white),
                  ],
                ),
              ),
              confirmDismiss: (direction) {
                if (direction == DismissDirection.startToEnd) {
                  triggerJisho(item.toString());
                } else {
                  triggerAnki(item.toString());
                }
                return Future.value(false);
              },
              child: ListTile(
                title: KanjiBankText(
                  text: item.toString(),
                  kanjiBank: kanjiBank,
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
