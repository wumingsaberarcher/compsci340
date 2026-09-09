#!/bin/sh
set -e

UPI="${1:-tli637}"
zip -r "CS340_A2_RUST_${UPI}.zip" kernel user
echo "Created CS340_A2_RUST_${UPI}.zip"
