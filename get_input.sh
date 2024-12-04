#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

curl -b "session=$SESSION_VALUE" "https://adventofcode.com/2024/day/"$1"/input" > ~/Misc/advent-of-code-2024/inputs/day-"$1".txt
