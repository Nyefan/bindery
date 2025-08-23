default:
  just --list
lint:
  echo "INFO:    Linting..."
  cargo fmt
  cargo clippy
build:
  echo "INFO:    Building..."
  cargo build --future-incompat-report --color always
test:
  echo "INFO:    Testing..."
  cargo test
run: build
  echo "INFO:    Running..."
  cargo run
release: lint build test
  echo "INFO:    Creating release build..."
  cargo build --future-incompat-report --color always --release --locked
build-d2-diagrams:
  d2 architecture/diagrams.d2