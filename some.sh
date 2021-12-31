#!/bin/bash
cd book
year=$1

if [ "$year" = "" ]; then
	year=[0-9][0-9][0-9][0-9]
fi

for d in $year; do
	echo $d
	tar Ccfz $d $d.tar.gz .
	time acurl -Fcontent=@${d}.tar.gz https://pages.sr.ht/publish/nanjj.srht.site/$d
done
