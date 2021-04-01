# pac

Package manager for vim/neovim using builtin `packages` written in rust.

## Install

Download and extract the precompiled binary from the releases page and put it somewhere in you `$PATH`

## Usage

By default `pac` will use `~/.vim/` for configuration and installing packages.
Set `$VIM_CONFIG_PATH` to `~/.config/nvim/` to use neovim instead.
`$VIM_CONFIG_PATH/.pac/paconfig.yaml` tracks the installed plugins and other
related configuration (commit this file to your dotfiles).

```bash
# general help
$ pac help
$ pac install -h

# install plugins
# pac install <github_user/github_repo>
$ pac install maralla/completor.vim
$ pac install maralla/completor.vim maralla/completor-neosnippet

# install all plugins
$ pac install

# install as optional plugin
$ pac install altercation/vim-colors-solarized -o

# install to a specific category
$ pac install pangloss/vim-javascript -c lang

# install a plugin which is loaded for a specifc filetype only
$ pac install maralla/rope.vim --for python
$ pac install mattn/emmet-vim --for html,jinja,xml

# install a plugin which is loaded for a specifc command only
$ pac install gregsexton/gitv --on Gitv

# install a plugin and build after installation (shell command only)
$ pac install Shougo/vimproc.vim --build 'make'

# list all installed packages
$ pac list

# uninstall a plugin
$ pac uninstall maralla/completor.vim
$ pac uninstall maralla/completor.vim maralla/completor-neosnippet

# update plugins
$ pac update
$ pac update maralla/completor.vim maralla/completor-neosnippet
```

## Shell completions

For bash, move `contrib/pac.bash` to `$XDG_CONFIG_HOME/bash_completion` or `/etc/bash_completion.d/`.

For fish, move `contrib/pac.fish` to `$HOME/.config/fish/completions/`.

For zsh, move `contrib/_pac` to one of your `$fpath` directories.

## License

Distributed under the terms of the [MIT](LICENSE) license.
