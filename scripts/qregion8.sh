#!/usr/bin/env bash

dir=$(mktemp -d)
# scrot -s -z -F $dir/color8.png
grim -g "$(slurp)" $dir/color8.png
magick $dir/color8.png \( +clone -evaluate set 0 -channel R -negate \) -compose multiply -composite $dir/cyan.png
magick $dir/color8.png \( +clone -evaluate set 0 -channel G -negate \) -compose multiply -composite $dir/magenta.png
magick $dir/color8.png \( +clone -evaluate set 0 -channel B -negate \) -compose multiply -composite $dir/yellow.png
zbarimg --raw -q -Sbinary $dir/cyan.png
zbarimg --raw -q -Sbinary $dir/magenta.png
zbarimg --raw -q -Sbinary $dir/yellow.png
shred -uz -n 25 $dir/*
rm -r $dir
