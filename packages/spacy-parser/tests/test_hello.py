"""Hello unit test module."""

from spacy_parser.hello import hello


def test_hello():
    """Test the hello function."""
    assert hello() == "Hello spacy-parser"
