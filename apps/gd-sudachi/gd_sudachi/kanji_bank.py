import json
import os
import platform
from abc import ABC
from typing import Generic, Literal, Optional, TypeGuard, TypeVar, TypedDict

Level = Literal["Apprentice", "Guru", "Master", "Enlightened", "Burned"]
KanjiType = Literal["kanji", "vocabulary"]

T = TypeVar("T")


class MetadataBase(ABC):
    pass


class KanjiData:
    onyomi_readings: list[str]
    kunyomi_readings: list[str]
    nanori_readings: list[str]


class VocabularyData:
    parts_of_speech: list[str]


class WaniKaniMetadata(MetadataBase):
    url: str
    level: str
    kanji_data: Optional[KanjiData] = None
    vocabulary_data: Optional[VocabularyData] = None


class KanjiBankEntry(Generic[T], TypedDict):
    level: Level
    type: KanjiType
    source: str  # Assuming KanjiSource is a string
    meaning: str
    metadata: T


KanjiBankData = dict[str, KanjiBankEntry[T]]
WaniKaniKanjiBankData = dict[str, KanjiBankEntry[WaniKaniMetadata]]


def is_wanikani_entry(
    entry: KanjiBankEntry,
) -> TypeGuard[KanjiBankEntry[WaniKaniMetadata]]:
    """Type guard to check if entry has WaniKani metadata"""
    return entry.get("source") == "wanikani"


def is_kanji(char):
    code = ord(char)
    return (
        (0x3400 <= code <= 0x4DBF)  # Extension A
        or (0x4E00 <= code <= 0x9FFF)  # Unified Ideographs
        or (0xF900 <= code <= 0xFAFF)  # Compatibility Ideographs
        or (0x20000 <= code <= 0x2A6DF)  # Extension B
        or (0x2A700 <= code <= 0x2B73F)  # Extension C
        or (0x2B740 <= code <= 0x2B81F)  # Extension D
        or (0x2B820 <= code <= 0x2CEAF)  # Extension E
        or (0x2CEB0 <= code <= 0x2EBEF)  # Extension F
        or (0x30000 <= code <= 0x3134F)  # Extension G
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
