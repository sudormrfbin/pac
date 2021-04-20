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

version-bump newver:
	#!/usr/bin/env bash
	set -euxo pipefail
	test "$(git branch --show-current)" = 'master'
	test -z "$(git status --porcelain)" # dirty index
	sed -i '0,/version/{s/version = "[^"]*"/version = "{{newver}}"/}' Cargo.toml
	sed -i '/## Unreleased/a \\n## {{newver}}' CHANGELOG.md
	cargo update
	git add CHANGELOG.md Cargo.toml Cargo.lock
	git commit -m 'build: Bump version to {{newver}}'
	git tag v{{newver}}
	git push origin v{{newver}}
