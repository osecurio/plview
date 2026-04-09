# mtkview

Will load GFH preloader binarie

Load MTK binaries:
- Preloaders
- LK (Todo)

## Build and install yourself

`git clone https://github.com/osecurio/mtkview`

`cd mtkview`

`DEP_BINARYNINJACORE_PATH=<PATH_TO_BINJA_DIR> cargo build --release`

`cp target/release/*.so ~/.binaryninja/plugins/`

## How to use

After building and installing, open Binary Ninja and select a partition or a raw MTK Preloader binary. The binja view should say `MTK`.

## Screenshot

![Demo](docs/screen.png)