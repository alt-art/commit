#!/bin/env bash

YELLOW='\e[1;33m'
NC='\e[0m'

if ! cargo fmt -- --check; then
    printf "%bHelp: run 'cargo fmt' to fix formatting issues%b" "$YELLOW" "$NC"
    exit 1
fi
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    printf "%bHelp: run 'cargo clippy' and fix the issues%b" "$YELLOW" "$NC"
    exit 1
fi
if ! cargo test; then
    printf "%bHelp: run 'cargo test' and fix the issues%b" "$YELLOW" "$NC"
    exit 1
fi
