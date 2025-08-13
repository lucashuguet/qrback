#!/usr/bin/env bash

dir=$(mktemp -d)
# scrot -s -z -F $dir/color4.png
grim -g "$(slurp)" $dir/color4.png
magick $dir/color4.png \( +clone -evaluate set 0 -channel R -negate -channel G -negate \) -compose multiply -composite $dir/blue.png
magick $dir/color4.png \( +clone -evaluate set 0 -channel B -negate \) -compose multiply -composite $dir/yellow.png
zbarimg --raw -q -Sbinary $dir/blue.png
zbarimg --raw -q -Sbinary $dir/yellow.png
shred -uz -n 25 $dir/*
rm -r $dir
