ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

#=================================================
# Bindings
#=================================================

bindings-index:
	cd generator; ./gradlew run --args="--directory=.. index"

bindings-update:
	cd generator; ./gradlew run --args="--directory=.. update --force --skip-upgrade"

#=================================================
# Book
#=================================================

cargo-mdbook:
	cargo install mdbook --version 0.4.21 --no-default-features

cargo-preprocessor:
	cargo install --path ./tutorial/book/preprocessor

book: bindings-index cargo-mdbook cargo-preprocessor
	cd ./tutorial/book; mdbook build

book-server: book
	/usr/bin/env python3 -m http.server 3000 --directory ./tutorial/book/book
