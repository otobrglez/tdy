#!/usr/bin/env bash
# Converts a Markdown note to PDF with pandoc and xelatex.
#
# Usage: ./convert.sh <input.md> <output.pdf>
#
# The font defaults to Inconsolata. Override it with TDY_FONT, for example:
#   TDY_FONT="Noto Sans" ./convert.sh notes.md notes.pdf
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <input.md> <output.pdf>" >&2
    exit 1
fi

from_file=$1
to_file=$2
font=${TDY_FONT:-inconsolata}
fallback_fonts="FallbackFonts={Noto Color Emoji,Noto Emoji,DejaVu Sans,Symbola}"

pandoc "$from_file" \
    --pdf-engine=xelatex \
    --variable geometry:a4paper,margin=2cm \
    --variable papersize:a4 \
    --variable mainfont="$font" \
    --variable sansfont="$font" \
    --variable monofont="$font" \
    --variable mathfont="STIX Two Math" \
    --variable mainfontoptions="$fallback_fonts" \
    --variable sansfontoptions="$fallback_fonts" \
    --variable monofontoptions="$fallback_fonts" \
    --variable fontsize=9pt \
    --variable version=2.0 \
    -o "$to_file"
