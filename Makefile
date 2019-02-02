.PHONY: all build pokered clean

all: build pokered

build:
	mkdir -p build

pokered:
	cd pokered && $(MAKE) red
	cp pokered/pokered.gbc build/
	cp pokered/pokered.sym build/

clean:
	rm -rf build
	cd pokered && $(MAKE) clean
