#!/usr/bin/env bash
set -ex

from_file=$1
to_file=$2

pandoc $from_file \
		-V geometry:a4paper,margin=2cm \
		-V fontsize=12pt \
		--variable mainfont="Palatino" \
		--variable sansfont="Helvetica" \
		--variable monofont="Menlo" \
		--variable fontsize=12pt \
		--variable version=2.0 \
		-o $to_file

# --variable "geometry=margin=1.2in" 
#		#-V "documentclass:extreport" \
#		-V 'mainfont:sans-serif' \

# 		-V 'fontfamily:Iosevka' \
#		-V 'sansfont:Iosevka' \
#		-V 'monofont:Iosevka' \
#		-V 'mathfont:TeXGyreDejaVuMath' \
