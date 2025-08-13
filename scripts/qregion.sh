#!/usr/bin/env bash

dir=$(mktemp -d)
# scrot -s -z -F $dir/qrcode.png
grim -g "$(slurp)" $dir/qrcode.png
zbarimg --raw -q -Sbinary $dir/qrcode.png
shred -uz -n 25 $dir/*
rm -r $dir
