# vim: ft=make

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

