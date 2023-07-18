#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

pushd "${SCRIPT_DIR}" || exit 1

pushd "${SCRIPT_DIR}/src/" || exit 1

ninja

popd || exit 1

cargo run --release --bin simics-fuzz --features=6.0.166 -- \
    -p test-project -c corpus -s solution -l TRACE  -C 1 \
    --package 2096:6.0.66 \
    --file "${SCRIPT_DIR}/src/mini.efi:%simics%/mini.efi" \
    --file "${SCRIPT_DIR}/rsrc/minimal_boot_disk.craff:%simics%/minimal_boot_disk.craff" \
    --file "${SCRIPT_DIR}/rsrc/fuzz.simics:%simics%/fuzz.simics" \
    --command 'COMMAND:run-script "%simics%/fuzz.simics"'
