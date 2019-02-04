# gitmoji-selector

Gitmojis taken from: [https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json]

# Installation

With the rust toolchain installed, navigate to the source directory (same as this README), then run the following command:

```sh
cargo install --path=. --force
```

# External dependencies

fzf with fzf-tmux. Also assumes you run this in tmux.

# Use in vim

ADd this line to your `.vimrc` or `init.vim`. With <Leader>j in normal mode,
you can select a gitmoji and insert it at the current cursor position.

```vimscript
nmap <Leader>j "=system('gitmoji-selector')<C-M>p
```

You can of course change what keys it is bound to.

Then visually select the spot you want to insert the gitmoji, and press the key combination.
