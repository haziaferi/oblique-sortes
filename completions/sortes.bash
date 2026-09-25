# bash completion for sortes.
#
# The deck list is not baked in: it comes from `sortes --list`, so a binary
# built with one deck completes one deck, and a deck added later needs no
# change here. Source this file, or drop it in your bash-completion directory.

# A deck line is one whose card count is a number. That is what separates a
# deck from the "* drawn from when no deck is named" footer, and it survives
# the marker on the default deck shifting the columns by one.
_sortes_decks() {
    "$1" --list 2> /dev/null | awk '{ if ($1 == "*") { id = $2; n = $3 } else { id = $1; n = $2 } ; if (n ~ /^[0-9]+$/) print id }'
}

_sortes() {
    local current previous options
    current="${COMP_WORDS[COMP_CWORD]}"
    previous="${COMP_WORDS[COMP_CWORD-1]}"

    # These take a value this script has no way to guess.
    case "$previous" in
        -n|--count|--seed|-f|--find) return 0 ;;
    esac

    options="-n --count --seed --any -a --all -f --find -l --list --about -h --help -V --version"

    if [[ "$current" == -* ]]; then
        mapfile -t COMPREPLY < <(compgen -W "$options" -- "$current")
    else
        mapfile -t COMPREPLY < <(compgen -W "$(_sortes_decks "${COMP_WORDS[0]}")" -- "$current")
    fi
}

complete -F _sortes sortes
