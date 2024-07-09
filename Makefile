CC = clang
GOARCH = amd64
GOPLATFORM = darwin
.PHONY: all cpu compiler
all: cpu compiler
cpu: 
	$(CC) -o build/cpu -L src/include src/init.c