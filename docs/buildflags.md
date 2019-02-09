# Build Flags

The current build system allows specifying a value for the Red ROM's build flags that indicates which features are
built into it. Running `make` from the parent directory will build all versions of the ROM in the `build` directory,
tagged with the value of the flags (e.g., `pokered3.gb`, where 3 is the value); building from the `pokered` directory
allows passing a value to `ROMFLAGS` directly (e.g., `make ROMFLAGS=4`) and assumes 0 otherwise.

Make sure to run `make clean` before changing the value of `ROMFLAGS` if building from the `pokered` directory.

The value is made of bit fields, as follows:

* bit 0: base game to build (clear: Red, set: Blue)
* bit 1: player character (clear: boy, set: girl)
* bit 2: item API enable flag

This gives the following combinations:

|`ROMFLAGS`|Base game|Gender|Item API|
|:--------:|:-------:|:----:|:------:|
|         0|Red      |Boy   |Not used|
|         1|Blue     |Boy   |Not used|
|         2|Red      |Girl  |Not used|
|         3|Blue     |Girl  |Not used|
|         4|Red      |Boy   |Enabled |
|         5|Blue     |Boy   |Enabled |
|         6|Red      |Girl  |Enabled |
|         7|Blue     |Girl  |Enabled |
