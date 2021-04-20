# vim: ft=make

list-commit-tags:
	@echo feat fix build chore ci docs style refactor perf test

install:
	cargo build --release
	cp {{justfile_directory()}}/target/release/pac ~/.local/bin/

test:
	cargo test

@lint:
	echo Checking for long lines...
	! grep --color -En '.{101}' src/**/*.rs
	echo Checking for FIXME/TODO...
	grep --color -Ein 'fixme|todo|xxx|#\[ignore\]' src/**/*.rs

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`
pac-targz := "pac-" + version + ".tar.gz"

gh-release:
	#!/usr/bin/env bash
	set -euxo pipefail
	cd build
	cargo build --release --target-dir .
	strip release/pac
	cp release/pac .
	tar cvzf {{pac-targz}} pac
	gh release create {{version}} {{pac-targz}} -R gokulsoumya/pac \
		--notes-file <(sed '/## [0-9]/,/## [0-9]/p' -n ../CHANGELOG.md | sed '1,2d; $d')
	rm -r *
