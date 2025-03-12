#!/usr/bin/env bash


if [ -n "$DEBUG" ]; then
	DEBUGFLAG="x"
else
	DEBUGFLAG=""
fi

set -euo"${DEBUGFLAG}" pipefail

echo "Building x86_64"
nix-build docker.nix -o result_x86_64

echo "Building aarch64"
nix-build docker-aarch64.nix -o result_aarch64
