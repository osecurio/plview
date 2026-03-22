# plview

## Build and install yourself

`git clone https://github.com/osecurio/plview`

`cd plview`

`DEP_BINARYNINJACORE_PATH=<PATH_TO_BINJA_DIR> cargo build --release`

`cp target/release/*.so ~/.binaryninja/plugins/`

## How to use

After building and installing, open Binary Ninja and select a partition or a raw MTK preloader binary. The binja view should say `MTK Preloader`, and there should be a header and code&data segment.