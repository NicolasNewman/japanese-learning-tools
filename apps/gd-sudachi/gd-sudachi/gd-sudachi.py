import argparse
import os

from kanji_bank import load_kanji_bank
from sudachi_helper import get_english_pos, get_english_pos_string
from sudachipy import Dictionary
from selectolax.parser import HTMLParser, Node

def create_logger(enabled: bool = False):
    def logger(*text):
        if enabled:
            print(*text)
    return logger

def main():
    parser = argparse.ArgumentParser(
        description="Parse Japanese text using SudachiPy and output HTML with POS tagging.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    ./gd-sudachi.bin "昨日買った3つのりんごを持っています。"
    ./gd-sudachi.bin -o output.html "日本語の文章を解析する"
        """,
    )
    kanji_bank = load_kanji_bank("com.nicolasnewman.jp-learning-tools")
    # log(len(kanji_bank.keys()))

    parser.add_argument(
        "text",
        nargs="?",
        default="昨日買った3つのりんごを持っています。",
        help="Japanese text to parse (default: example sentence)",
    )

    parser.add_argument(
        "--debug", "-d",
        action="store_true",
        help="Enable debug logging"
    )

    args = parser.parse_args()

    # Get the bundle directory for resources
    if "__compiled__" in globals():
        # We're in a Nuitka bundle
        bundle_dir = os.path.dirname(os.path.abspath(__file__))
        dict_path = os.path.join(
            bundle_dir, "sudachidict_core", "resources", "system.dic"
        )
        os.environ["SUDACHIDICT_CORE_PATH"] = dict_path

        tokenizer_obj = Dictionary(dict=dict_path).create()
    else:
        tokenizer_obj = Dictionary().create()


    original_html = """
    <a href="/wiki/%E3%83%A1%E3%82%A4%E3%83%B3%E3%83%9A%E3%83%BC%E3%82%B8" title="メインページ">メイン<b>ページ</b></a>の<b>今日の一枚</b>では、一日一枚の画像を紹介しています。今日の一枚は、<a href="/wiki/Wikipedia:%E7%A7%80%E9%80%B8%E3%81%AA%E7%94%BB%E5%83%8F" title="Wikipedia:秀逸な画像">秀逸な画像</a>に選ばれたものの中から掲載されています。今日の一枚に掲載したい優れた画像があるときは、<a href="/wiki/Wikipedia:%E7%A7%80%E9%80%B8%E3%81%AA%E7%94%BB%E5%83%8F%E3%81%AE%E6%8E%A8%E8%96%A6" title="Wikipedia:秀逸な画像の推薦">Wikipedia:秀逸な画像の推薦</a>にて推薦してみてください。
    """
    tree = HTMLParser(args.text)

    log = create_logger(args.debug)

    text = tree.text(True, "", True)

    html_map: list[Node]= []
    if (tree.root):
        for node in tree.root.traverse(True):
            if node.tag != "html" and node.tag != "body" and node.tag != "head":
                if not node.child:
                    html_map.append(node)
                    log(f"Text: {node.text(False)}")

    tokens = tokenizer_obj.tokenize(text)

    accumulator_html = 0
    html_idx = 0
    html_text = html_map[html_idx].text(True, "", True)
    updated_html = html_text
    updated_html_start_shift = 0
    updated_html_end_shift = 0
    for token in tokens:
        character = token.surface()
        normalized_form = token.normalized_form()
        # 3rd = counter?
        bank_data = kanji_bank.get(normalized_form)
        (pos, pos_sub, _, _, conj_type, conj_form) = get_english_pos(token).values()

        log(character, get_english_pos_string(token), normalized_form, bank_data)
        accumulator_sudachi = 0
        html_start_rel_sudachi = accumulator_html
        for _sudachi_i in range(len(character)):
            for _html_i in range(len(html_text)):
                log("\t", character[accumulator_sudachi], html_text[accumulator_html])
                if character[accumulator_sudachi] != html_text[accumulator_html]:
                    log("\tNO MATCH")
                log("\t", accumulator_sudachi + 1, len(character), " : ", accumulator_html + 1, len(html_text))
                accumulator_sudachi += 1
                if accumulator_html == len(html_text) - 1:
                    if updated_html != html_text:
                        log("\tREPLACING HTML")
                        html_map[html_idx].replace_with(HTMLParser(updated_html).body) # type: ignore
                    if html_idx >= len(html_map) - 1:
                        log("\t\tBREAKING HTML")
                        break
                    html_idx += 1
                    html_text = html_map[html_idx].text(True, "", True)
                    accumulator_html = 0
                    updated_html = html_text
                    updated_html_start_shift = 0
                    updated_html_end_shift = 0
                    log("\tNext HTML node: ", html_text)
                else:
                    accumulator_html += 1
                if accumulator_sudachi >= len(character):
                    log("\t\tBREAKING SUDACHI")
                    break
            if accumulator_sudachi >= len(character):
                if bank_data:
                    log("\t\t\t", updated_html[html_start_rel_sudachi+updated_html_start_shift:accumulator_html+updated_html_end_shift])
                    opening_tag = f'<span class="{pos} {bank_data.get("source")} {bank_data.get("type")} stage-{bank_data.get("level").lower()}">'
                    closing_tag = f"</span>"
                    updated_html = updated_html[:html_start_rel_sudachi+updated_html_start_shift] + f"{opening_tag}{character}{closing_tag}" + updated_html[accumulator_html+updated_html_end_shift:]
                    updated_html_start_shift += len(opening_tag) + len(closing_tag)
                    updated_html_end_shift += len(closing_tag)  + len(opening_tag)
                    log("\t\t\t", updated_html)
                log("\t\tBREAKING SUDACHI")
                break
    if (tree.body and tree.body.html):
        text = tree.body.html.replace("<body>", "").replace("</body>", "")
        print(text)


if __name__ == "__main__":
    main()
