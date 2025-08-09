#!/usr/bin/env bash
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <problem_name>"
    exit 1
fi

PROBLEM_NAME="$1"
PROBLEM_DIR="problems/$PROBLEM_NAME"

# Create crate
cargo new --bin "$PROBLEM_DIR"

# Add to workspace Cargo.toml if not already present
WORKSPACE_TOML="Cargo.toml"
if ! grep -q "\"$PROBLEM_DIR\"" "$WORKSPACE_TOML"; then
    # Insert before the last closing bracket in members array
    sed -i.bak "/members = \[/a\ \ \ \ \"$PROBLEM_DIR\"," "$WORKSPACE_TOML"
    rm "$WORKSPACE_TOML.bak"
    echo "Added $PROBLEM_NAME to workspace."
fi

cat > "$PROBLEM_DIR/src/main.rs" <<'EOF'
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    println!("{}", solve());
}

fn next<T: std::str::FromStr>(iter: &mut std::str::SplitWhitespace<'_>) -> T {
    iter.next().unwrap().parse().ok().unwrap()
}

fn solve() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(solve(), 42);
    }
}
EOF


echo "Created $PROBLEM_DIR"

