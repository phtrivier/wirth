#!/usr/bin/env sh
MDS="book/00_cover.md book/01_intro.md book/02_scanner.md"
verso src/scanner2.rs | recto target $MDS
cd target
echo $MDS
ls -al
ls -al book/
pandoc --number-sections -s --toc -o book.pdf $MDS
