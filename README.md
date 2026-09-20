# cargo-churn

Measure total lines added and deleted across all commits in a Git repository.

## Requirements

- Rust 1.85 or newer
- Git

## Install

Install the binary from a local checkout:

```sh
git clone https://github.com/todaymare/cargo-churn.git
cd cargo-churn
cargo install --path .
```

## Usage

Run `cargo-churn` in the repository you want to measure, or pass a repository path:

```sh
cargo-churn
cargo-churn /path/to/repository
```

The output reports total text-file additions and deletions across all commits reachable from all Git refs:

```text
+12k / -3k
```

Use `--ignore` to exclude files whose paths match a regular expression. The option can be repeated:

```sh
cargo-churn /path/to/repository \
  --ignore 'Cargo.lock' \
  --ignore '\.snap$'
```

Binary files are skipped because Git does not provide line counts for them.

## Development

```sh
cargo fmt --check
cargo test
cargo run -- --help
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE).
