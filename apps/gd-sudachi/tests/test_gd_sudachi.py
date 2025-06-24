import sys
import json
import pytest
from unittest.mock import patch
import importlib

class TestGdSudachi:
    @pytest.fixture(autouse=True)
    def setup(self, monkeypatch):
        """Setup common test fixtures that run before each test method"""
        # Load the module just once
        self.main_module = importlib.import_module("gd_sudachi.main")
        
        # Load kanji bank
        with open("tests/mocks/kanji-bank.json", encoding="utf-8") as f:
            self.kanji_bank = json.load(f)
        
        # Create tokenizer
        from sudachipy import Dictionary
        self.tokenizer = Dictionary().create()
        
        # Patch the module
        monkeypatch.setattr(self.main_module, "load_kanji_bank", lambda _: self.kanji_bank)
        monkeypatch.setattr(Dictionary, "create", lambda cls: self.tokenizer)
    
    def run_test(self, monkeypatch, capsys, input_html, expected_output):
        """Helper method to run a test with given input and expected output"""
        # Set command line arguments
        sys_argv = ["gd-sudachi.py", input_html]
        monkeypatch.setattr(sys, "argv", sys_argv)
        
        # Import and run main
        from gd_sudachi import main
        main()
        
        # Check output
        out, err = capsys.readouterr()
        out = out.rstrip('\n')
        assert out == expected_output

    def test_wikipedia_example1(self, monkeypatch, capsys):
        """Test with Wikipedia-style HTML"""
        input_html = '<a href="/wiki/%E3%83%A1%E3%82%A4%E3%83%B3%E3%83%9A%E3%83%BC%E3%82%B8" title="メインページ">メイン<b>ページ</b></a>の<b>今日の一枚</b>では、一日一枚の画像を紹介所しています。今日の一枚は、<a href="/wiki/Wikipedia:%E7%A7%80%E9%80%B8%E3%81%AA%E7%94%BB%E5%83%8F" title="Wikipedia:秀逸な画像">秀逸な画像</a>に選ばれたものの中から掲載されています。今日の一枚に掲載したい優れた画像があるときは、<a href="/wiki/Wikipedia:%E7%A7%80%E9%80%B8%E3%81%AA%E7%94%BB%E5%83%8F%E3%81%AE%E6%8E%A8%E8%96%A6" title="Wikipedia:秀逸な画像の推薦">Wikipedia:秀逸な画像の推薦</a>にて推薦してみてください。'
        expected_output = '<a href="/wiki/%E3%83%A1%E3%82%A4%E3%83%B3%E3%83%9A%E3%83%BC%E3%82%B8" title="メインページ">メイン<b>ページ</b></a>の<b><span class="noun wanikani vocabulary stage-enlightened">今日</span>の<span class="noun wanikani kanji stage-enlightened">一</span>枚</b>では、<span class="noun wanikani kanji stage-enlightened">一</span><span class="noun wanikani vocabulary stage-master">日</span><span class="noun wanikani kanji stage-enlightened">一</span>枚の<span class="noun wanikani vocabulary stage-apprentice">画像</span>を紹介<span class="noun wanikani kanji stage-master">所</span>しています。<span class="noun wanikani vocabulary stage-enlightened">今日</span>の<span class="noun wanikani kanji stage-enlightened">一</span>枚は、<a href="/wiki/Wikipedia:%E7%A7%80%E9%80%B8%E3%81%AA%E7%94%BB%E5%83%8F" title="Wikipedia:秀逸な画像">秀逸な<span class="noun wanikani vocabulary stage-apprentice">画像</span></a>に選ばれた<span class="noun wanikani vocabulary stage-master">もの</span>の<span class="noun wanikani vocabulary stage-enlightened">中</span>から掲載されています。<span class="noun wanikani vocabulary stage-enlightened">今日</span>の<span class="noun wanikani kanji stage-enlightened">一</span>枚に掲載したい優れた<span class="noun wanikani vocabulary stage-apprentice">画像</span>が<span class="verb wanikani vocabulary stage-enlightened">ある</span><span class="noun wanikani vocabulary stage-apprentice">とき</span>は、<a href="/wiki/Wikipedia:%E7%A7%80%E9%80%B8%E3%81%AA%E7%94%BB%E5%83%8F%E3%81%AE%E6%8E%A8%E8%96%A6" title="Wikipedia:秀逸な画像の推薦">Wikipedia:秀逸な<span class="noun wanikani vocabulary stage-apprentice">画像</span>の推薦</a>にて推薦して<span class="verb wanikani vocabulary stage-enlightened">み</span>てください。'
        
        self.run_test(monkeypatch, capsys, input_html, expected_output)

    def test_wikipedia_example2(self, monkeypatch, capsys):
        """Test with Wikipedia-style HTML"""
        input_html = '<b><a href="/wiki/%E5%85%89%E5%8E%B3%E5%A4%A9%E7%9A%87" title="光厳天皇">光厳天皇</a></b>は、<a href="/wiki/%E6%97%A5%E6%9C%AC" title="日本">日本</a>の<a href="/wiki/%E5%8C%97%E6%9C%9D_(%E6%97%A5%E6%9C%AC)" title="北朝 (日本)">北朝</a>現代<a href="/wiki/%E5%A4%A9%E7%9A%87" title="天皇">天皇</a>。<a href="/wiki/%E8%AB%B1" title="諱">諱</a>は<b>量仁</b>。<a href="/wiki/%E5%BE%8C%E4%BC%8F%E8%A6%8B%E5%A4%A9%E7%9A%87" title="後伏見天皇">後伏見天皇</a>の第三皇子。母は<a href="/wiki/%E5%B7%A6%E5%A4%A7%E8%87%A3" title="左大臣">左大臣</a><a href="/wiki/%E8%A5%BF%E5%9C%92%E5%AF%BA%E5%85%AC%E8%A1%A1" title="西園寺公衡">西園寺公衡</a>の娘で後伏見の<a href="/wiki/%E5%A5%B3%E5%BE%A1" title="女御">女御</a>の<a href="/wiki/%E8%A5%BF%E5%9C%92%E5%AF%BA%E5%AF%A7%E5%AD%90" title="西園寺寧子">西園寺寧子</a>。<a href="/wiki/%E5%BE%8C%E9%86%8D%E9%86%90%E5%A4%A9%E7%9A%87" title="後醍醐天皇">後醍醐天皇</a>によって<a href="/wiki/%E5%BB%83%E4%BD%8D" class="mw-redirect" title="廃位">廃位</a>されたが、<a href="/wiki/%E5%BB%BA%E6%AD%A6%E6%94%BF%E6%A8%A9" class="mw-redirect" title="建武政権">建武政権</a>崩壊後に<a href="/wiki/%E6%B2%BB%E5%A4%A9%E3%81%AE%E5%90%9B" title="治天の君">治天の君</a>となって北朝を主導し、<a href="/wiki/%E5%AE%A4%E7%94%BA%E5%B9%95%E5%BA%9C" title="室町幕府">室町幕府</a>との<a href="/wiki/%E5%BE%B3%E6%94%BF%E4%BB%A4#徳政" title="徳政令">公武徳政</a>や『<a href="/wiki/%E9%A2%A8%E9%9B%85%E5%92%8C%E6%AD%8C%E9%9B%86" title="風雅和歌集">風雅和歌集</a>』の親撰などを行った。<a href="/wiki/%E5%A4%A9%E9%BE%8D%E5%AF%BA" title="天龍寺">天龍寺</a>や<a href="/wiki/%E5%AE%89%E5%9B%BD%E5%AF%BA%E5%88%A9%E7%94%9F%E5%A1%94" title="安国寺利生塔">安国寺利生塔</a>の建立にも関与している。<a href="/wiki/%E6%AD%A3%E5%B9%B3%E4%B8%80%E7%B5%B1" class="mw-redirect" title="正平一統">正平一統</a>が破綻した際、<a href="/wiki/%E5%8D%97%E6%9C%9D_(%E6%97%A5%E6%9C%AC)" title="南朝 (日本)">南朝</a>によって<a href="/wiki/%E6%8B%89%E8%87%B4" title="拉致">拉致</a>された。'
        expected_output = '<b><a href="/wiki/%E5%85%89%E5%8E%B3%E5%A4%A9%E7%9A%87" title="光厳天皇"><span class="noun wanikani kanji stage-enlightened">光</span>厳<span class="noun wanikani kanji stage-enlightened">天</span>皇</a></b>は、<a href="/wiki/%E6%97%A5%E6%9C%AC" title="日本"><span class="noun wanikani vocabulary stage-enlightened">日本</span></a>の<a href="/wiki/%E5%8C%97%E6%9C%9D_(%E6%97%A5%E6%9C%AC)" title="北朝 (日本)"><span class="noun wanikani kanji stage-enlightened">北</span><span class="noun wanikani kanji stage-master">朝</span></a>現<span class="noun wanikani kanji stage-enlightened">代</span><a href="/wiki/%E5%A4%A9%E7%9A%87" title="天皇"><span class="noun wanikani kanji stage-enlightened">天</span>皇</a>。<a href="/wiki/%E8%AB%B1" title="諱">諱</a>は<b>量仁</b>。<a href="/wiki/%E5%BE%8C%E4%BC%8F%E8%A6%8B%E5%A4%A9%E7%9A%87" title="後伏見天皇"><span class="noun wanikani kanji stage-enlightened">後</span>伏<span class="noun wanikani kanji stage-enlightened">見</span><span class="noun wanikani kanji stage-enlightened">天</span>皇</a>の第<span class="noun wanikani kanji stage-enlightened">三</span>皇<span class="noun wanikani kanji stage-enlightened">子</span>。<span class="noun wanikani vocabulary stage-enlightened">母</span>は<a href="/wiki/%E5%B7%A6%E5%A4%A7%E8%87%A3" title="左大臣"><span class="noun wanikani kanji stage-enlightened">左</span><span class="noun wanikani kanji stage-enlightened">大</span>臣</a><a href="/wiki/%E8%A5%BF%E5%9C%92%E5%AF%BA%E5%85%AC%E8%A1%A1" title="西園寺公衡"><span class="noun wanikani kanji stage-enlightened">西</span>園寺<span class="noun wanikani kanji stage-guru">公</span>衡</a>の娘で<span class="noun wanikani kanji stage-enlightened">後</span>伏<span class="noun wanikani kanji stage-enlightened">見</span>の<a href="/wiki/%E5%A5%B3%E5%BE%A1" title="女御"><span class="noun wanikani kanji stage-enlightened">女</span>御</a>の<a href="/wiki/%E8%A5%BF%E5%9C%92%E5%AF%BA%E5%AF%A7%E5%AD%90" title="西園寺寧子"><span class="noun wanikani kanji stage-enlightened">西</span>園寺寧<span class="noun wanikani kanji stage-enlightened">子</span></a>。<a href="/wiki/%E5%BE%8C%E9%86%8D%E9%86%90%E5%A4%A9%E7%9A%87" title="後醍醐天皇"><span class="noun wanikani kanji stage-enlightened">後</span>醍醐<span class="noun wanikani kanji stage-enlightened">天</span>皇</a>によって<a href="/wiki/%E5%BB%83%E4%BD%8D" class="mw-redirect" title="廃位">廃<span class="noun wanikani kanji stage-apprentice">位</span></a>されたが、<a href="/wiki/%E5%BB%BA%E6%AD%A6%E6%94%BF%E6%A8%A9" class="mw-redirect" title="建武政権">建武政権</a>崩壊<span class="suffix wanikani kanji stage-enlightened">後</span>に<a href="/wiki/%E6%B2%BB%E5%A4%A9%E3%81%AE%E5%90%9B" title="治天の君">治<span class="noun wanikani kanji stage-enlightened">天</span>の<span class="代名詞 wanikani vocabulary stage-master">君</span></a>となって<span class="noun wanikani kanji stage-enlightened">北</span><span class="noun wanikani kanji stage-master">朝</span>を<span class="noun wanikani kanji stage-enlightened">主</span>導し、<a href="/wiki/%E5%AE%A4%E7%94%BA%E5%B9%95%E5%BA%9C" title="室町幕府"><span class="noun wanikani kanji stage-enlightened">室</span><span class="noun wanikani kanji stage-enlightened">町</span>幕府</a>との<a href="/wiki/%E5%BE%B3%E6%94%BF%E4%BB%A4#徳政" title="徳政令"><span class="noun wanikani kanji stage-guru">公</span>武徳政</a>や『<a href="/wiki/%E9%A2%A8%E9%9B%85%E5%92%8C%E6%AD%8C%E9%9B%86" title="風雅和歌集"><span class="noun wanikani kanji stage-master">風</span>雅<span class="noun wanikani kanji stage-master">和</span><span class="noun wanikani kanji stage-guru">歌</span><span class="noun wanikani kanji stage-guru">集</span></a>』の<span class="noun wanikani vocabulary stage-guru">親</span>撰などを<span class="verb wanikani vocabulary stage-master">行っ</span>た。<a href="/wiki/%E5%A4%A9%E9%BE%8D%E5%AF%BA" title="天龍寺"><span class="noun wanikani kanji stage-enlightened">天</span>龍寺</a>や<a href="/wiki/%E5%AE%89%E5%9B%BD%E5%AF%BA%E5%88%A9%E7%94%9F%E5%A1%94" title="安国寺利生塔"><span class="noun wanikani kanji stage-guru">安</span><span class="noun wanikani kanji stage-enlightened">国</span>寺<span class="noun wanikani kanji stage-apprentice">利</span><span class="noun wanikani kanji stage-enlightened">生</span>塔</a>の建<span class="noun wanikani kanji stage-enlightened">立</span>にも関与している。<a href="/wiki/%E6%AD%A3%E5%B9%B3%E4%B8%80%E7%B5%B1" class="mw-redirect" title="正平一統"><span class="noun wanikani kanji stage-enlightened">正</span><span class="noun wanikani kanji stage-enlightened">平</span><span class="noun wanikani kanji stage-enlightened">一</span>統</a>が破綻した際、<a href="/wiki/%E5%8D%97%E6%9C%9D_(%E6%97%A5%E6%9C%AC)" title="南朝 (日本)"><span class="noun wanikani kanji stage-enlightened">南</span><span class="noun wanikani kanji stage-master">朝</span></a>によって<a href="/wiki/%E6%8B%89%E8%87%B4" title="拉致">拉致</a>された。'
        
        self.run_test(monkeypatch, capsys, input_html, expected_output)
    
    def test_simple_example(self, monkeypatch, capsys):
        """Test with simple Japanese text"""
        input_html = "昨日買った3つのりんごを持っています。"
        expected_output = '昨<span class="noun wanikani kanji stage-master">日</span><span class="verb wanikani vocabulary stage-master">買っ</span>た3つのりんごを<span class="verb wanikani vocabulary stage-apprentice">持っ</span>ています。'
        
        self.run_test(monkeypatch, capsys, input_html, expected_output)