enum DictionaryBackendType {
  jisho('Jisho'),
  takoboto('Takoboto');

  final String label;
  const DictionaryBackendType(this.label);

  @override
  String toString() => label;

  String getSearchUrl(String searchTerm) {
    switch (this) {
      case DictionaryBackendType.jisho:
        return 'https://jisho.org/search/$searchTerm';
      case DictionaryBackendType.takoboto:
        return 'https://takoboto.jp/?q=$searchTerm';
    }
  }
}
