# fish completion for sortes.
#
# `sortes --list` is the source of the deck names, so this file does not go
# stale when a deck is added, and a single-deck build completes a single deck.
#
# A deck line is one whose card count is a number: that separates a deck from
# the footer, and survives the marker on the default deck shifting the columns.

function __sortes_decks
    command sortes --list 2> /dev/null | awk '{ if ($1 == "*") { id = $2; n = $3 } else { id = $1; n = $2 } ; if (n ~ /^[0-9]+$/) print id }'
end

complete -c sortes -f
complete -c sortes -n __fish_use_subcommand -a "(__sortes_decks)" -d Deck

complete -c sortes -s n -l count   -r -d 'Draw N cards, without replacement'
complete -c sortes      -l seed    -r -d 'Draw reproducibly from this seed'
complete -c sortes      -l any        -d 'Draw from a deck picked at random'
complete -c sortes -s a -l all        -d 'Print every card in the deck, in order'
complete -c sortes -s f -l find    -r -d 'Print every card containing TEXT'
complete -c sortes -s l -l list       -d 'List the decks built into this binary'
complete -c sortes      -l about      -d 'Describe a deck and say where its text came from'
complete -c sortes -s h -l help       -d 'Print the grammar'
complete -c sortes -s V -l version    -d 'Print the version'
