#!/usr/bin/env bash
set -ex

from_file=$1
to_file=$2

font_0="inconsolata"
font_1="Palatino"
font_2="Menlo"
font_3="Helvetica"

font=$font_0
mainfont=$font
sansfont=$font
monofont=$font

pandoc $from_file \
    --pdf-engine=xelatex \
		-V geometry:a4paper,margin=2cm \
    --variable mainfont="$mainfont" \
    --variable sansfont="$sansfont" \
    --variable monofont="$monofont" \
    -V mainfontoptions="FallbackFonts={Symbola}" \
    -V sansfontoptions="FallbackFonts={Symbola}" \
    -V monofontoptions="FallbackFonts={Symbola}" \
		--variable fontsize=10pt \
		--variable version=2.0 \
		-o $to_file
