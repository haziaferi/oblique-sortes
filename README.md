# Oblique Strategies

Brian Eno and Peter Schmidt's [Oblique Strategies](https://en.wikipedia.org/wiki/Oblique_Strategies) in a library, for those moments when the work is stuck and a dilemma needs a lateral nudge.

The list is a curated amalgam of the card text from several editions of the deck, so it includes variant wordings of some strategies (and one blank white card, as the deck itself does). Cards that have several lines of text contain embedded newline and tab characters. Print them as-is and they will render the way the card reads:

```text
>  oblique
Destroy
	-nothing
	-the most important thing
```

## Usage

```rust
let strategy = oblique::random();
println!("The next move: {}", strategy);
```

If you are using the cli:

```text
>  oblique
Honor thy error as a hidden intention
```

## See also

[gsv-culture-ships](https://github.com/ceejbot/gsv-culture-ships), this library's structural twin for Iain M. Banks's Culture ship names, another thing I always keep handy for process ping responses.

## LICENSE

[ISC](./LICENSE.md). Go ahead and steal this one, o freeloading megacorps.
