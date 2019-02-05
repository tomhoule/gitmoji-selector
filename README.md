# gitmoji-selector

Gitmojis taken from: https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json

# Installation

With the rust toolchain installed

- if you have the source navigate to the source directory (same as this
  README), then run the following command:

```sh
cargo install --path=. --force
```

- directly from git: cargo install --git=https://github.com/tomhoule/gitmoji-selector --force

# External dependencies

fzf with fzf-tmux. Also assumes you run this in tmux.

# Use in vim

Add this line to your `.vimrc` or `init.vim`. With `<Leader>j` in normal mode,
you can select a gitmoji and insert it at the current cursor position.

```vimscript
nmap <Leader>j "=system('gitmoji-selector')<C-M>P
```

You can of course change what keys it is bound to.
