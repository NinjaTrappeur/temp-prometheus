CC=rustc

build:
	$(CC) -O main.rs -o temp-prometheus

install: build
	install -d $(PREFIX)/bin/
	install -m 755 ./temp-prometheus $(PREFIX)/bin/
