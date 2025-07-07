import base64
import json

try:
    from .sudachi_mapping import (
        conj_form_mapping,
        conj_type_mapping,
        pos1_mapping,
        pos2_mapping,
        pos3_mapping,
        pos4_mapping,
    )
except ImportError:
    from sudachi_mapping import (
        conj_form_mapping,
        conj_type_mapping,
        pos1_mapping,
        pos2_mapping,
        pos3_mapping,
        pos4_mapping,
    )


def encode_to_b64(data) -> str:
    """Convert any JSON-serializable data to Base64 string"""
    json_string = json.dumps(data, ensure_ascii=False)
    return base64.b64encode(json_string.encode("utf-8")).decode("utf-8")


def get_english_pos(token):
    """
    Get English translations for all grammatical fields of a SudachiPy token.

    Args:
        token: SudachiPy token object

    Returns:
        dict: Dictionary with all fields translated to English
    """
    # Get the part-of-speech tuple from the token
    pos_tuple = token.part_of_speech()

    # Map each element to its English equivalent
    results = {
        "pos": pos1_mapping.get(pos_tuple[0], pos_tuple[0]) if pos_tuple[0] else "NULL",
        "pos_subcategory1": pos2_mapping.get(pos_tuple[1], pos_tuple[1])
        if pos_tuple[1]
        else "NULL",
        "pos_subcategory2": pos3_mapping.get(pos_tuple[2], pos_tuple[2])
        if pos_tuple[2]
        else "NULL",
        "pos_subcategory3": pos4_mapping.get(pos_tuple[3], pos_tuple[3])
        if pos_tuple[3]
        else "NULL",
        "conjugation_type": conj_type_mapping.get(pos_tuple[4], pos_tuple[4])
        if pos_tuple[4]
        else "NULL",
        "conjugation_form": conj_form_mapping.get(pos_tuple[5], pos_tuple[5])
        if pos_tuple[5]
        else "NULL",
    }

    return results


def get_english_pos_string(token):
    """
    Get a single string with all translated grammatical information.

    Args:
        token: SudachiPy token object

    Returns:
        str: Comma-separated string of English grammatical information
    """
    info = get_english_pos(token)
    return f"({", ".join(v for v in info.values())})"


def extend_token_class(token_class):
    """
    Extend the SudachiPy token class with English POS properties.

    This may not work directly with SudachiPy as it depends on the implementation,
    but provides a pattern if extension is supported.
    """
    token_class.english_pos_dict = property(lambda self: get_english_pos(self))
    token_class.english_pos = property(lambda self: get_english_pos_string(self))
