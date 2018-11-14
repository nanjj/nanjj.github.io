#!/bin/bash

function mkdir-p {
    local p=$(dirname $1)
    if [ -d $p ]; then
      return
    fi
    mkdir -p $p
}

for f in `find . -type f -name '*.css'|grep -v 'family'`; do
    echo $f...
    urls=$(cat $f | grep '@import url(http' | cut -d'(' -f2 | cut -d')' -f1)
    if [ -z "$urls" ]; then
        continue
    fi
    for url in $urls; do
        echo $url
        name="css/$(echo $url|cut -d'?' -f2|sed 's/[^a-zA-Z0-9]/-/g').css"
        if [ ! "${name}" -nt $0 ]; then
            echo $name
            mkdir-p $name
            wget -O "$name" "$url"
        fi
        fonts=$(cat $name | grep 'url(http' | sed "s/.*url(\(http.*.ttf\)).*/\1/g")
        if [ -n "$fonts" ]; then
            for font in $fonts; do
                echo $font
                fname="$(echo $font|cut -d/ -f4-)"
                fpath="css/$fname"
                if [ ! -f $fpath ]; then
                    mkdir-p $fpath
                    wget -O "$fpath" "$font"
                fi
                sed -e "s;$font;$fname;g" -i $name
            done
        fi
        sed -e "s;$url;../../$name;g" -i "$f"
    done
done
