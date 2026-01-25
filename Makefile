GODOT := godot

LINUX_PRESET := "Linux"
WINDOWS_PRESET := "Windows"

BUILD_DIR := "build"

.PHONY: all linux windows clean presets

all: linux

presets:
	$(GODOT) --headless --export-list

linux:
	mkdir -p $(BUILD_DIR)/linux
	$(GODOT) --headless --export-release "$(LINUX_PRESET)" $(BUILD_DIR)/linux/game.x86_64

windows:
	mkdir -p $(BUILD_DIR)/windows
	$(GODOT) --headless --export-release "$(WINDOWS_PRESET)" $(BUILD_DIR)/windows/game.exe

clean:
	rm -rf $(BUILD_DIR)
