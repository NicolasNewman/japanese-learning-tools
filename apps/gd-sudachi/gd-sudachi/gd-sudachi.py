import os
import argparse
from sudachipy import tokenizer, Dictionary
from sudachi_helper import get_english_pos, get_english_pos_string
import sys

def main():
    parser = argparse.ArgumentParser(
        description="Parse Japanese text using SudachiPy and output HTML with POS tagging.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    ./gd-sudachi.bin "昨日買った3つのりんごを持っています。"
    ./gd-sudachi.bin -o output.html "日本語の文章を解析する"
        """
    )

    parser.add_argument("text", nargs="?", default="昨日買った3つのりんごを持っています。",
                        help="Japanese text to parse (default: example sentence)")
    
    args = parser.parse_args()

    # Get the bundle directory for resources
    if '__compiled__' in globals():
        # We're in a Nuitka bundle
        bundle_dir = os.path.dirname(os.path.abspath(__file__))
        dict_path = os.path.join(bundle_dir, 'sudachidict_core', 'resources', 'system.dic')
        os.environ['SUDACHIDICT_CORE_PATH'] = dict_path

        tokenizer_obj = Dictionary(dict=dict_path).create()
    else:
        tokenizer_obj = Dictionary().create()

    tokens = tokenizer_obj.tokenize(args.text)

    new_text = ""
    for token in tokens:
        character = token.surface()
        normalized_form = token.normalized_form()
        # 3rd = counter?
        (pos, pos_sub, _, _, conj_type, conj_form) = get_english_pos(token).values()
        print(token.surface(), get_english_pos_string(token), token.normalized_form())
        if pos == "noun":
            new_text += f"<span class=\"{pos}\">{character}</span>"
        elif pos == "verb" or pos == "adjective":
            new_text += f"<span class=\"{pos}\">{character}</span>"
        else:
            new_text += character
    print(new_text)

if __name__ == "__main__":
    main()