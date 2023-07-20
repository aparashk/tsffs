#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

if ! command -v fd &>/dev/null; then
    echo "fd must be installed! Install with 'cargo install fd-find'"
    exit 1
fi

if ! command -v flake8 &>/dev/null; then
    echo "flake8 must be installed! Install with 'python3 -m pip install flake8'"
    exit 1
fi

if ! command -v pylint &>/dev/null; then
    echo "pylint must be installed! Install with 'python3 -m pip install pylint'"
    exit 1
fi

if ! command -v mypy &>/dev/null; then
    echo "pylint must be installed! Install with 'python3 -m pip install pylint'"
    exit 1
fi

if ! command -v cargo &>/dev/null; then
    echo "cargo must be installed! Install from https://rustup.rs"
    exit 1
fi

if ! command -v yamllint &>/dev/null; then
    echo "yamllint must be installed! Install from your package manager."
    exit 1
fi

if ! command -v markdownlint &>/dev/null; then
    echo "markdownlint must be installed! Install with 'npm i -g markdownlint-cli'"
    exit 1
fi

echo "================="
echo "Running clippy..."
echo "================="

cargo clippy --features=6.0.167

echo "================="
echo "Running flake8..."
echo "================="

fd '.*\.py$' -x flake8 --config "${SCRIPT_DIR}/../.github/linters/.flake8" {}

echo "================="
echo "Running mypy..."
echo "================="

fd '.*\.py$' -x mypy --config-file "${SCRIPT_DIR}/../.github/linters/.mypy.ini" {}

echo "================="
echo "Running pylint..."
echo "================="

fd '.*\.py$' -x pylint {}

echo "================="
echo "Running yamllint..."
echo "================="

fd '.*(\.yml|\.yaml)$' -x yamllint -c "${SCRIPT_DIR}/../.github/linters/.yaml-lint.yml" {}

echo "================="
echo "Running markdownlint..."
echo "================="

fd '.*\.md$' -x markdownlint -c "${SCRIPT_DIR}/../.github/linters/.markdown-lint.yml" {}
