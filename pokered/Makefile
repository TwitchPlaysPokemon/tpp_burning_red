pokered_obj := audio.o main.o text.o wram.o

### Build tools

MD5 := md5sum -c

RGBDS ?=
RGBASM  ?= $(RGBDS)rgbasm
RGBFIX  ?= $(RGBDS)rgbfix
RGBGFX  ?= $(RGBDS)rgbgfx
RGBLINK ?= $(RGBDS)rgblink

### Build targets

.SUFFIXES:
.SECONDEXPANSION:
.PRECIOUS:
.SECONDARY:
.PHONY: all clean tidy tools

roms := pokered.gbc

ROMFLAGS := 0

all: $(roms)

clean:
	rm -f $(roms) $(pokered_obj) $(roms:.gbc=.sym)
	find . \( -iname '*.1bpp' -o -iname '*.2bpp' -o -iname '*.pic' \) -exec rm {} +
	$(MAKE) clean -C tools/

tidy:
	rm -f $(roms) $(pokered_obj) $(roms:.gbc=.sym)
	$(MAKE) clean -C tools/

tools:
	$(MAKE) -C tools/


# Build tools when building the rom.
# This has to happen before the rules are processed, since that's when scan_includes is run.
ifeq (,$(filter clean tools,$(MAKECMDGOALS)))
$(info $(shell $(MAKE) -C tools))
endif


%.asm: ;

%.o: dep = $(shell tools/scan_includes $(@D)/$*.asm)
%.o: %.asm $$(dep)
	$(RGBASM) -h -D_BUILD_FLAGS=$(ROMFLAGS) -o $@ $*.asm

ifeq (1,$(shell echo $$((1 & $(ROMFLAGS)))))
pokered_opt := -jsv -k 01 -l 0x33 -m 0x13 -p 0 -r 03 -t "POKEMON BLUE"
else
pokered_opt := -jsv -k 01 -l 0x33 -m 0x13 -p 0 -r 03 -t "POKEMON RED"
endif

pokered.gbc: $(pokered_obj)
	$(RGBLINK) -d -n pokered.sym -l pokered.link -o $@ $^
	$(RGBFIX) $(pokered_opt) $@
	sort pokered.sym -o pokered.sym


### Misc file-specific graphics rules

gfx/blue/intro_purin_1.2bpp: $(RGBGFX) += -h
gfx/blue/intro_purin_2.2bpp: $(RGBGFX) += -h
gfx/blue/intro_purin_3.2bpp: $(RGBGFX) += -h
gfx/red/intro_nido_1.2bpp: $(RGBGFX) += -h
gfx/red/intro_nido_2.2bpp: $(RGBGFX) += -h
gfx/red/intro_nido_3.2bpp: $(RGBGFX) += -h

gfx/game_boy.2bpp: tools/gfx += --remove-duplicates
gfx/theend.2bpp: tools/gfx += --interleave --png=$<
gfx/tilesets/%.2bpp: tools/gfx += --trim-whitespace


### Catch-all graphics rules

%.png: ;

%.2bpp: %.png
	$(RGBGFX) $(rgbgfx) -o $@ $<
	$(if $(tools/gfx),\
		tools/gfx $(tools/gfx) -o $@ $@)

%.1bpp: %.png
	$(RGBGFX) -d1 $(rgbgfx) -o $@ $<
	$(if $(tools/gfx),\
		tools/gfx $(tools/gfx) -d1 -o $@ $@)

%.pic:  %.2bpp
	tools/pkmncompress $< $@
