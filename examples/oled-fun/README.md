To convert GIF to BMPs (will need ImageMagick):

```
cd .include
convert noo-nooo.gif -coalesce temp_coalesced.gif
convert temp_coalesced.gif -resize 48x48 \
    -edge 1 -threshold 8% \
    frame-%d.bmp

```