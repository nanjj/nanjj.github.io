all:
	mdbook build
	rm -rf book/*.rs
	cp -rf book/* .

publish:
	tar Ccfz book site.tar.gz .
	acurl -Fcontent=@site.tar.gz https://pages.sr.ht/publish/nanjj.srht.site
