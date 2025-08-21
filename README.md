# japanese-learning-tools

A collection of tools to aid in Japanese immersion-based learning

## gd-tools

> A rust port of [gd-tools](https://github.com/Ajatt-Tools/gd-tools) by Ajatt-Tools. 

gd-tools aims to provide a set of programs to enhance GoldenDict for immersion learning.

## subs2clipboard

A Firefox extention which provides 2 features:

1. Automatically copy subtitles from supported streaming services directly to your clipboard (which triggers an automatic search in GoldenDict)
    - Supported services: Jellyfin, YouTube
2. Highlight Kanji on a webpage based on the stages in supported SRS programs (using gd-sudachi)
    - Supported SRS: Anki, WaniKani

> Note: At the moment, I have no desire to support Chrome & Manifest v3.

## gd-sudachi

A tool for highlighting Japanese text based on what you know from your SRS

## desktop application

A desktop application which handles automatically seting up gd-tools/subs2clipboard on installation and syncs up your SRS' with subs2clipboard