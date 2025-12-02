#!/usr/bin/env bash
set -ex

from_file=$1
to_file=$2

font_0="inconsolata"
font_1="Palatino"
font_2="Menlo"
font_3="Helvetica"
font_4="Noto Sans"

font=$font_4
mainfont=$font
sansfont=$font
monofont=$font

pandoc $from_file \
    --pdf-engine=xelatex \
		-V geometry:a4paper,margin=2cm \
    --variable mainfont="$mainfont" \
    --variable sansfont="$sansfont" \
    --variable monofont="$monofont" \
    --variable mathfont="STIX Two Math" \
    -V mainfontoptions="FallbackFonts={Noto Color Emoji,Noto Emoji,DejaVu Sans,Symbola}" \
    -V sansfontoptions="FallbackFonts={Noto Color Emoji,Noto Emoji,DejaVu Sans,Symbola}" \
    -V monofontoptions="FallbackFonts={Noto Color Emoji,Noto Emoji,DejaVu Sans,Symbola}" \
		--variable fontsize=10pt \
		--variable version=2.0 \
		-o $to_file

# pandoc $from_file \
#   --pdf-engine=lualatex \
#   -V geometry:a4paper,margin=2cm \
#   -V mainfont="Noto Sans" \
#   -V sansfont="Noto Sans" \
#   -V monofont="Inconsolata" \
#   -V mathfont="STIX Two Math" \
#   -V 'mainfontoptions=Renderer=Harfbuzz,FallbackFonts={Noto Color Emoji,Noto Emoji,DejaVu Sans}' \
#   -V 'sansfontoptions=Renderer=Harfbuzz,FallbackFonts={Noto Color Emoji,Noto Emoji,DejaVu Sans}' \
#   -V 'monofontoptions=Renderer=Harfbuzz,FallbackFonts={Noto Emoji,DejaVu Sans Mono}' \
#   -V fontsize=10pt \
# 	-o $to_file
