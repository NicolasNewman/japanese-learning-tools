import json
import os
import platform
from typing import Literal, TypedDict

KanjiType = Literal["kanji", "vocabulary"]

class KanjiSource(TypedDict, total=False):
    # Define the fields of KanjiSource here, for example:
    # source_name: str
    # source_url: str
    pass  # Replace with actual fields

class KanjiBankEntry(TypedDict):
    level: str
    type: KanjiType
    source: KanjiSource

KanjiBankData = dict[str, KanjiBankEntry]

def is_kanji(char):
    code = ord(char)
    return (
        (0x3400 <= code <= 0x4DBF) or    # Extension A
        (0x4E00 <= code <= 0x9FFF) or    # Unified Ideographs
        (0xF900 <= code <= 0xFAFF) or    # Compatibility Ideographs
        (0x20000 <= code <= 0x2A6DF) or  # Extension B
        (0x2A700 <= code <= 0x2B73F) or  # Extension C
        (0x2B740 <= code <= 0x2B81F) or  # Extension D
        (0x2B820 <= code <= 0x2CEAF) or  # Extension E
        (0x2CEB0 <= code <= 0x2EBEF) or  # Extension F
        (0x30000 <= code <= 0x3134F)     # Extension G
    )

def get_app_data_dir(bundle_identifier: str) -> str:
    home = os.path.expanduser("~")
    system = platform.system()
    if system == "Linux":
        base = os.path.join(home, ".local", "share")
    elif system == "Darwin":
        base = os.path.join(home, "Library", "Application Support")
    elif system == "Windows":
        base = os.environ.get("APPDATA", os.path.join(home, "AppData", "Roaming"))
    else:
        raise RuntimeError(f"Unsupported OS: {system}")
    return os.path.join(base, bundle_identifier)

def load_kanji_bank(bundle_identifier: str) -> KanjiBankData:
    app_data_dir = get_app_data_dir(bundle_identifier)
    file_path = os.path.join(app_data_dir, "kanji-bank.json")
    with open(file_path, encoding="utf-8") as f:
        return json.load(f)