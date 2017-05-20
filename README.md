
# xor-genkeys
Command line application that generates keys that can be used in guesses when attempting to decrypt content encrypted using xor encryption.

### Installation

If you've not already done so, install rust:
https://www.rust-lang.org/

Then install via cargo with:
```bash
$ cargo install xor-genkeys
```

### Help
```bash
$ xor-genkeys --help
xor-genkeys 0.1.0
Gavyn Riebau
Generates sets of ascii values that can be used as guessed keys when decrypting xor encrypted content

USAGE:
	xor-genkeys [LENGTH]

FLAGS:
	-h, --help       Prints help information
	-V, --version    Prints version information

ARGS:
	<LENGTH>    The assumed key length [default: 1]
```

### Example

To create a file containing all the possible 3 character length ascii keys:

```bash
$ xor-genkeys 3 > keys.txt
```
