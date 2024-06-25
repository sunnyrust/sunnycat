BUILD_NAME      := sunnycat
BUILD_TIME      := $(shell date "+%F %T")
BUILD_VERSION   := 🛎V0.1.5-$(shell date "+%Y%m%d")🛎


#SOURCE          := main.go
#TARGET_DIR      := ./
GIT_VERSION      := $(shell git rev-parse HEAD )
EMOJI           := _|￣|○ -----🎉🎉🎉👍💁👌   DLang$(BUILD_NAME)  ⚽🎍😍🎉🎉🎉------○|￣|_


debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

.PHONY : all clean install ${BUILD_NAME}

hello:
	@echo "\e[1;33mGit Version:\e[1;35m$(GIT_VERSION))."; \


build:
	@echo "\e[1;33mGit Version:\e[1;35m$(GIT_VERSION)."; \
	sed -i 's/gitversion="[^"]*"/gitversion="$(GIT_VERSION)"/g' Cargo.toml
	cargo build $(release)

install:
	@echo "\e[1;33m$(BUILD_NAME)  \e[1;35m  install."
	@if [ "$(debug)" = "debug" ]; then \
		cp target/$(target)/$(BUILD_NAME) /usr/local/bin/$(BUILD_NAME)-$(extension); \
	else \
		cp target/$(target)/$(BUILD_NAME) /usr/local/bin/$(BUILD_NAME); \
	fi

all: build install

#all:
#	sed -i -E "s/(description=\")+(\")/\1$(GIT_VERSION)\2/" Cargo.toml
#	cargo build --release
#clean:
#	rm ${BUILD_NAME} -f
#
#install:
#	mkdir -p ${TARGET_DIR}
#	cp ${BUILD_NAME} ${TARGET_DIR} -f
