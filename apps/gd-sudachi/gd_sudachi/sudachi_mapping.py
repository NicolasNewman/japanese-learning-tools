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
    "空白": "whitespace",
    "代名詞": "pronoun",
    "形状詞": "predicative",
    "*": "*",
}

# 2. POS2 subcategory
pos2_mapping = {
    "普通名詞": "common noun",
    "一般": "general",
    "格助詞": "case particle",
    "接続助詞": "conjunctive particle",
    "名詞的": "nominal",
    "形容詞的": "adjectival",
    "形状詞的": "predicative",
    "動詞的": "verbal",
    "数詞": "numeral",
    "非自立可能": "auxiliary",
    "句点": "period",
    "読点": "comma",
    "括弧開": "open bracket",
    "括弧閉": "close bracket",
    "係助詞": "binding particle",
    "固有名詞": "proper noun",
    "終助詞": "final particle",
    "準体助詞": "pre-nominal particle",
    "助動詞語幹": "auxiliary stem",
    "副助詞": "adverbial particle",
    "*": "*",
}

# 3. POS3 subcategory
pos3_mapping = {
    "副詞可能": "adverbial",
    "一般": "general",
    "助数詞": "counter",
    "人名": "person name",
    "助動詞語幹": "auxiliary stem",
    "サ変可能": "suru-verb possible",
    "助数詞可能": "counter possible",
    "形状詞可能": "predicative possible",
    "*": "*",
}

# 4. POS4 subcategory
pos4_mapping = {
    "名": "name",
    "一般": "general",
    "*": "*",
    # Add more as needed
}

# 5. Conjugation type
conj_type_mapping = {
    "五段-ワア行": "godan-wa-a",
    "五段-タ行": "godan-ta",
    "五段-ラ行": "godan-ra",
    "五段-サ行": "godan-sa",
    "五段-カ行": "godan-ka",
    "五段-ナ行": "godan-na",
    "五段-バ行": "godan-ba",
    "五段-マ行": "godan-ma",
    "上一段-ア行": "upper-ichidan-a",
    "上一段-カ行": "upper-ichidan-ka",
    "下一段-ナ行": "lower-ichidan-na",
    "下一段-バ行": "lower-ichidan-ba",
    "下一段-サ行": "lower-ichidan-sa",
    "下一段-ガ行": "lower-ichidan-ga",
    "下一段-ラ行": "lower-ichidan-ra",
    "下一段-マ行": "lower-ichidan-ma",
    "下一段-タ行": "lower-ichidan-ta",
    "下一段-カ行": "lower-ichidan-ka",
    "下一段-ア行": "lower-ichidan-a",
    "助動詞-タ": "auxiliary-ta",
    "助動詞-ダ": "auxiliary-da",
    "助動詞-タイ": "auxiliary-tai",
    "助動詞-マス": "auxiliary-masu",
    "助動詞-レル": "auxiliary-reru",
    "助動詞-ナイ": "auxiliary-nai",
    "助動詞-デス": "auxiliary-desu",
    "カ行変格": "ka-irregular",
    "サ行変格": "sa-irregular",
    "形容詞": "adjective",
    "*": "*",
}

# 6. Conjugation form
conj_form_mapping = {
    "語幹-一般": "stem-general",
    "連用形-促音便": "continuative-geminated",
    "連用形-融合": "continuative-fused",
    "連用形-一般": "continuative-normal",
    "連用形-ニ": "continuative-ni",
    "連用形-撥音便": "continuative-nasalized",
    "連用形-イ音便": "continuative-i-euphonic",
    "連体形-一般": "attributive-normal",
    "終止形-一般": "terminal-normal",
    "未然形-一般": "imperfective-normal",
    "仮定形-一般": "conditional-normal",
    "意志推量形": "volitional-inferential",
    "命令形": "imperative",
    "*": "*",
}
