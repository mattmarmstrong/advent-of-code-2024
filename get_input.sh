#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

SESSION_VALUE="53616c7465645f5fd6740b995815cbff5b761eaa03859d2a4cc54831f60859cdb2e3d6ac9d0a90e5665275c073a31adb0fedd04a47f562dc2857888c9f5670cc"

curl -b "session=$SESSION_VALUE" "https://adventofcode.com/2024/day/"$1"/input" > ~/Misc/advent-of-code-2024/inputs/day-"$1".txt
