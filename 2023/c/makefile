
BUILD_DIR := build

all: ${BUILD_DIR}/day1
clean:
	rm -rf ${BUILD_DIR}

${BUILD_DIR}:
	@mkdir -p build

${BUILD_DIR}/day1: day1.c ${BUILD_DIR}
	cc -Wall -Werror -Wextra -std=c11 -o $@ $<
