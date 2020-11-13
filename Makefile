BUILD_NAME      := sunnycat
BUILD_TIME      := $(shell date "+%F %T")
BUILD_VERSION   := 🛎V0.1.0-$(shell date "+%Y%m%d")🛎


SOURCE          := main.go
TARGET_DIR      := ./
GIT_VERSION      := $(shell git rev-parse HEAD )
EMOJI           := _|￣|○ -----🎉🎉🎉👍💁👌   DLang$(BUILD_NAME)  ⚽🎍😍🎉🎉🎉------○|￣|_

all:
	sed -i -E "s/(description=\")+(\")/\1$(GIT_VERSION)\2/" Cargo.toml 
	cargo build
clean:
	rm ${BUILD_NAME} -f

install:
	mkdir -p ${TARGET_DIR}
	cp ${BUILD_NAME} ${TARGET_DIR} -f

.PHONY : all clean install ${BUILD_NAME}