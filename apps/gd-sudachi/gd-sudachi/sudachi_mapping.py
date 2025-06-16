"""
Mapping of SudachiPy grammatical categories to English translations.
"""

# 1. Main part of speech (POS1)
pos1_mapping = {
    "名詞": "noun",
    "動詞": "verb",
    "助動詞": "auxiliary verb",
    "補助記号": "symbol",
    "接尾辞": "suffix",
    "助詞": "particle",
    "形容詞": "adjective",
    "接頭辞": "prefix",
    "副詞": "adverb",
    "連体詞": "adnominal",
    "接続詞": "conjunction",
    "感動詞": "interjection",
    "*": "*"
}

# 2. POS2 subcategory
pos2_mapping = {
    "普通名詞": "common noun",
    "一般": "general",
    "格助詞": "case particle",
    "接続助詞": "conjunctive particle",
    "名詞的": "nominal",
    "数詞": "numeral",
    "非自立可能": "auxiliary",
    "句点": "period",
    "読点": "comma",
    "括弧開": "open bracket",
    "括弧閉": "close bracket",
    "*": "*"
}

# 3. POS3 subcategory
pos3_mapping = {
    "副詞可能": "adverbial",
    "一般": "general",
    "助数詞": "counter",
    "助動詞語幹": "auxiliary stem",
    "*": "*"
}

# 4. POS4 subcategory
pos4_mapping = {
    "*": "*"
    # Add more as needed
}

# 5. Conjugation type
conj_type_mapping = {
    "五段-ワア行": "godan-wa-a",
    "五段-タ行": "godan-ta",
    "上一段-ア行": "upper-ichidan-a",
    "助動詞-タ": "auxiliary-ta",
    "助動詞-マス": "auxiliary-masu",
    "*": "*"
}

# 6. Conjugation form
conj_form_mapping = {
    "連用形-促音便": "continuative-geminated",
    "連体形-一般": "attributive-normal",
    "連用形-一般": "continuative-normal",
    "終止形-一般": "terminal-normal",
    "*": "*"
}