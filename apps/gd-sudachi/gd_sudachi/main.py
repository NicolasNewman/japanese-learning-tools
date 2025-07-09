import contextlib
import locale
import sys

# Add this at the very beginning of main.py, before any other imports
if sys.platform == "win32":
    # Force UTF-8 encoding on Windows
    sys.stdout.reconfigure(encoding="utf-8")
    sys.stderr.reconfigure(encoding="utf-8")
    sys.stdin.reconfigure(encoding="utf-8")
    # Set locale to handle Japanese characters properly
    try:
        locale.setlocale(locale.LC_ALL, "Japanese_Japan.UTF-8")
    except locale.Error:
        try:
            locale.setlocale(locale.LC_ALL, "ja_JP.UTF-8")
        except locale.Error:
            contextlib.suppress(locale.Error)

import argparse
import json
import os

try:
    # When running as a package (normal usage and tests)
    from .kanji_bank import is_kanji, load_kanji_bank
    from .sudachi_helper import encode_to_b64, get_english_pos, get_english_pos_string
except ImportError:
    # When bundled with Nuitka or run as script
    from kanji_bank import is_kanji, load_kanji_bank
    from sudachi_helper import encode_to_b64, get_english_pos, get_english_pos_string

from selectolax.parser import HTMLParser, Node
from sudachipy import Dictionary


def create_logger(enabled: bool = False):
    def logger(*text):
        if enabled:
            print(*text)

    return logger


def create_panic(enabled: bool = False):
    def panic(*text):
        if enabled:
            print(*text)
            exit(1)

    return panic


def create_tokenizer():
    """Create and return a SudachiPy tokenizer object"""
    # Get the bundle directory for resources
    if "__compiled__" in globals():
        # We're in a Nuitka bundle
        bundle_dir = os.path.dirname(os.path.abspath(__file__))
        dict_path = os.path.join(
            bundle_dir, "sudachidict_core", "resources", "system.dic"
        )
        os.environ["SUDACHIDICT_CORE_PATH"] = dict_path
        return Dictionary(dict=dict_path).create()
    else:
        return Dictionary().create()


def process_text_to_html(text, tokenizer_obj, kanji_bank, debug=False):
    """Process text and return HTML with POS tagging"""
    tree = HTMLParser(text)

    log = create_logger(debug)
    panic = create_panic(debug)

    text_content = tree.text(True, "", True)

    html_map: list[Node] = []
    if tree.root:
        for node in tree.root.traverse(True):
            if node.tag not in ["html", "body", "head"] and not node.child:
                html_map.append(node)
                log(f"Text: {node.text(False)}")

    tokens = tokenizer_obj.tokenize(text_content)

    # Initialize the HTML char index accumulator and the current HTML node index
    html_accumulator = 0
    html_node_index = 0
    html_text = html_map[html_node_index].text(True, "", True)
    updated_html = html_text
    updated_html_start_shift = 0
    updated_html_end_shift = 0

    # Loop through each token from the SudachiPy tokenizer
    for token in tokens:
        # The current SudachiPy token
        sudachi_text = token.surface()
        normalized_form = token.normalized_form()
        bank_data = kanji_bank.get(normalized_form)
        (pos, pos_sub, _, _, conj_type, conj_form) = get_english_pos(token).values()

        log(sudachi_text, get_english_pos_string(token), normalized_form, bank_data)
        sudachi_accumulator = 0
        # As html is appended to the original text, the index of each html character are shifted by the length of the HTML tags added
        html_start_rel_sudachi = html_accumulator
        for _sudachi_i in range(len(sudachi_text)):
            for _html_i in range(len(html_text)):
                log(
                    "\t", sudachi_text[sudachi_accumulator], html_text[html_accumulator]
                )
                # The character from SudachiPy and the current HTML character should always match for each iteration
                if sudachi_text[sudachi_accumulator] != html_text[html_accumulator]:
                    panic("NO MATCH FOUND")
                log(
                    "\t",
                    sudachi_accumulator + 1,
                    len(sudachi_text),
                    " : ",
                    html_accumulator + 1,
                    len(html_text),
                )
                sudachi_accumulator += 1

                # If we've reached the end of the current SudachiPy token
                if sudachi_accumulator >= len(sudachi_text):
                    # and the full token is stored in the kanji bank,
                    # surround the token in the HTML with a styled span
                    if bank_data:
                        # html_accumulator shifted by 1 (off-by-one error)
                        tmp_accumulator_html = html_accumulator + 1
                        log(
                            "\t\t\tBank Vocab Found:",
                            updated_html[
                                html_start_rel_sudachi
                                + updated_html_start_shift : tmp_accumulator_html
                                + updated_html_end_shift
                            ],
                        )
                        opening_tag = f'<span data-source="{bank_data.get("source")}" data-meaning="{bank_data.get("meaning")}" data-metadata={encode_to_b64(bank_data.get("metadata"))} class="{pos} {bank_data.get("source")} {bank_data.get("type")} stage-{bank_data.get("level").lower()}">'
                        closing_tag = "</span>"
                        log("\t\t\t", updated_html)
                        updated_html = (
                            updated_html[
                                : html_start_rel_sudachi + updated_html_start_shift
                            ]
                            + f"{opening_tag}{sudachi_text}{closing_tag}"
                            + updated_html[
                                tmp_accumulator_html + updated_html_end_shift :
                            ]
                        )
                        updated_html_start_shift += len(opening_tag) + len(closing_tag)
                        updated_html_end_shift += len(closing_tag) + len(opening_tag)
                        log("\t\t\t", updated_html)
                    # otherwise, check if each character in the SudachiPy token is a kanji that is stored in the bank
                    else:
                        for i, char in enumerate(sudachi_text):
                            if is_kanji(char):
                                individual_bank_data = kanji_bank.get(char)
                                if individual_bank_data:
                                    # accumulator_html shifted to the current kanji index
                                    tmp_accumulator_html = (
                                        html_accumulator + i - (len(sudachi_text) - 2)
                                    )
                                    tmp_range = updated_html[
                                        html_start_rel_sudachi
                                        + updated_html_start_shift
                                        + i : tmp_accumulator_html
                                        + updated_html_end_shift
                                    ]
                                    log(f"\t\t\tBank Kanji Found: {char}/{tmp_range}")
                                    opening_tag = f'<span data-source="{individual_bank_data.get("source")}" data-meaning="{individual_bank_data.get("meaning")}" data-metadata={encode_to_b64(individual_bank_data.get("metadata"))} class="{pos} {individual_bank_data.get("source")} kanji stage-{individual_bank_data.get("level").lower()}">'
                                    closing_tag = "</span>"
                                    log("\t\t\t", updated_html)
                                    updated_html = (
                                        updated_html[
                                            : html_start_rel_sudachi
                                            + updated_html_start_shift
                                            + i
                                        ]
                                        + f"{opening_tag}{char}{closing_tag}"
                                        + updated_html[
                                            tmp_accumulator_html
                                            + updated_html_end_shift :
                                        ]
                                    )
                                    updated_html_start_shift += len(opening_tag) + len(
                                        closing_tag
                                    )
                                    updated_html_end_shift += len(closing_tag) + len(
                                        opening_tag
                                    )
                                    log("\t\t\t", updated_html)
                                    if len(tmp_range) != 1:
                                        panic(
                                            f"Only 1 character not in range: {tmp_range}"
                                        )

                # If the html accumulator has reached the end of the current HTML node,
                # replace the HTML node with the updated HTML
                # and advance to the next HTML node
                if html_accumulator == len(html_text) - 1:
                    if updated_html != html_text:
                        log("\tREPLACING HTML")
                        html_map[html_node_index].replace_with(
                            HTMLParser(updated_html).body
                        )  # type: ignore
                    if html_node_index >= len(html_map) - 1:
                        log("\t\tBREAKING HTML")
                        break
                    html_node_index += 1
                    html_text = html_map[html_node_index].text(True, "", True)
                    html_accumulator = 0
                    updated_html = html_text
                    updated_html_start_shift = 0
                    updated_html_end_shift = 0
                    log("  Next HTML node: ", html_text)
                else:
                    html_accumulator += 1

                if sudachi_accumulator >= len(sudachi_text):
                    break
            if sudachi_accumulator >= len(sudachi_text):
                break
    if tree.body and tree.body.html:
        result = tree.body.html.replace("<body>", "").replace("</body>", "")
        return result
    return ""


def run_as_daemon():
    """Run as a daemon that processes JSON requests from stdin"""

    # Initialize everything ONCE at startup
    sys.stderr.write("Daemon starting - initializing tokenizer...\n")
    sys.stderr.flush()

    try:
        kanji_bank = load_kanji_bank("com.nicolasnewman.jp-learning-tools")

        # Get the bundle directory for resources
        tokenizer_obj = create_tokenizer()

        sys.stderr.write("Daemon ready - waiting for requests...\n")
        sys.stderr.flush()

        # Process requests in a loop
        for line in sys.stdin:
            try:
                # Parse request
                request = json.loads(line.strip())
                text = request.get("text", "")
                debug = request.get("debug", False)

                if not text:
                    continue

                # Process text
                result = process_text_to_html(text, tokenizer_obj, kanji_bank, debug)

                # Send response back as JSON
                response = {"status": "success", "result": result}
                print(json.dumps(response))
                sys.stdout.flush()

            except json.JSONDecodeError:
                error_response = {"status": "error", "message": "Invalid JSON"}
                print(json.dumps(error_response))
                sys.stdout.flush()

            except Exception as e:
                error_response = {"status": "error", "message": str(e)}
                print(json.dumps(error_response))
                sys.stdout.flush()

    except KeyboardInterrupt:
        sys.stderr.write("Daemon shutting down...\n")
        sys.stderr.flush()
    except Exception as e:
        sys.stderr.write(f"Daemon initialization failed: {e}\n")
        sys.stderr.flush()
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(
        description="Parse Japanese text using SudachiPy and output HTML with POS tagging.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    ./gd-sudachi.bin "昨日買った3つのりんごを持っています。"
    ./gd-sudachi.bin -o output.html "日本語の文章を解析する"
    ./gd-sudachi.bin --daemon  # Run as daemon for persistent processing
        """,
    )

    parser.add_argument(
        "text",
        nargs="?",
        default="昨日買った3つのりんごを持っています。",
        help="Japanese text to parse (default: example sentence)",
    )

    parser.add_argument(
        "--debug", "-d", action="store_true", help="Enable debug logging"
    )

    parser.add_argument(
        "--daemon", action="store_true", help="Run as daemon for persistent processing"
    )

    args = parser.parse_args()

    # If daemon mode is requested, run as daemon
    if args.daemon:
        run_as_daemon()
        return

    # Original single-run mode
    kanji_bank = load_kanji_bank("com.nicolasnewman.jp-learning-tools")

    # Get the bundle directory for resources
    tokenizer_obj = create_tokenizer()

    # Process single text input
    result = process_text_to_html(args.text, tokenizer_obj, kanji_bank, args.debug)
    print(result)


if __name__ == "__main__":
    main()
