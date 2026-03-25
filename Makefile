PROFILE ?= debug
BIN_NAME := impactor

ifeq ($(PROFILE),release)
CARGO_PROFILE := --release
TARGET_DIR := release
else
CARGO_PROFILE :=
TARGET_DIR := debug
endif

clean:
	@rm -rf dist/
	@rm -rf target/

build:
	@cargo build -p $(BIN_NAME) $(CARGO_PROFILE)

dist: build
	@mkdir -p dist
	@cp target/$(TARGET_DIR)/$(BIN_NAME) dist/$(BIN_NAME)