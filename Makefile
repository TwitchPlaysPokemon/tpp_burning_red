.PHONY: all build pokered clean

all: build pokered

build:
	mkdir -p build

pokered:
	./pokered.sh

clean:
	rm -rf build
	cd pokered && $(MAKE) clean
