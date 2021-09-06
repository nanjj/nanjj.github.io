all: build
	rm book/*.rs
	cp -rf book/* .
build: clean
	mdbook build
clean:
	mdbook clean
serve:
	mdbook serve

