#!/usr/bin/env bash
set -euo pipefail

echo "=== ZoKrates General-purpose ZK Proof ==="
command -v zokrates
zokrates --help >/dev/null

echo
echo "1. Compiling circuit..."
zokrates compile -i square.zok

echo
echo "2. Running setup..."
zokrates setup

echo
echo "3. Computing witness..."
# state=5, operation=1 (multiply), p1=25, p2=0, p3=0
# public expected result=125
zokrates compute-witness -a 5 1 25 0 0 125

echo
echo "4. Generating proof..."
zokrates generate-proof

echo
echo "5. Verifying proof..."
zokrates verify

echo
echo "Proof generated and verified!"