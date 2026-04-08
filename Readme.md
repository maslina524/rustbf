# RUSTBF

Experimental project as a first compiler

## Usage

```console
rustbf <file_path> <args>
```

## Args

**`-t`** — don't delete `.c` temp file

**`-r`** — run exe

## Example

```console
rustbf tests/hello_world.bf -r
```

## Build

```console
C:\...\rustbf> cargo build --release
```