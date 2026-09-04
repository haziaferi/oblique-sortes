# Shell completions

Deck names complete from `sortes --list`, not from a list kept in these files.
A binary built with one deck completes one deck, and a deck added to the crate
needs no change here.

| Shell | File | Where it goes |
|---|---|---|
| bash | `sortes.bash` | source it, or drop it in your bash-completion directory |
| zsh | `_sortes` | anywhere on your `$fpath` |
| fish | `sortes.fish` | `~/.config/fish/completions/` |

These ship in the release tarball too, in a `completions/` directory beside the binary.

The one thing these files know about the output of `--list` is that a deck line
carries a card count as a number, which is what tells a deck apart from the
`* drawn from when no deck is named` footer and survives the marker on the
default deck shifting the columns by one. Change the shape of `--list` and
these three `awk` expressions are what to change with it.
