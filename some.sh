#!/bin/bash
cd book
for d in [0-9][0-9][0-9][0-9]; do
	echo $d
	tar Ccfz $d $d.tar.gz .
	time acurl -Fcontent=@${d}.tar.gz https://pages.sr.ht/publish/nanjj.srht.site/$d
done
