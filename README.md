# javadoc2adoc

This project searches in a given glob path pattern for java files. It will then try to extract all [Javadoc](https://en.wikipedia.org/wiki/Javadoc) entries, sort-of turn them into [AsciiDoc](https://asciidoc.org/) and write them to files, matching the path structure of the found files.

## How to build

```bash
cargo build --release
```

## How to run

```bash
javadoc2adoc -i '<glob-pattern>' -o <out-dir>
```

Hint: Escape the glob pattern so your shell doesn't expand it.

Example:

```bash
javadoc2adoc -i './**/*.java' -o tmp
```
	
## What works

- Classes
- Constructors
- Methods
- Fields
- Nested Classes
- Interfaces

This can handle the [Quarkus Repo](https://github.com/quarkusio/quarkus) without crashing, which is kind of nice.

## What doesn't work

- Most likely a lot of things

## Why would you want this

You most likely don't; This just annoyed me.
