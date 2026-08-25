#!/usr/bin/env bash
#
# Run the golden render matrix and print a pass/fail table.
#
#   ./scripts/run_test_matrix.sh              # all 21 cases
#   ./scripts/run_test_matrix.sh store        # only cases matching /store/
#   ./scripts/run_test_matrix.sh 'bar\.|visual'
#
# This is the supported way to run the matrix. Before it existed the
# only way in was a hand-written instantiation under /tmp, because the
# package takes callPackage arguments and this repo's flake exports no
# configurations to reach them through. See ../tests/matrix.nix for the
# door itself; this script is the handle.
#
# Why a script and not a flake output: this repo is a library. Its
# flake exports `lib`, `out` and one installer image, and defines no
# host configurations on purpose -- wrappers own machine identity.
# Adding a checks/packages tree for the matrix would mean instantiating
# a system here to get pkgs from, which is the line the flake is drawn
# to avoid. `out.pkgs` is the escape hatch it documents instead, and it
# is all the matrix needs.
#
# --------------------------------------------------------------------
# The fetcher, and why the URL below is built the way it is
#
# `git+file:` -- NEVER `path:`. The path fetcher copies a directory into
# the store wholesale and does not honour .gitignore; that is
# git+file:'s behaviour, not its own. A working checkout of this crate
# carries a multi-gigabyte cargo `target/`, the store path is
# content-addressed, and a tree under active editing changes between
# every run, so each run mints another multi-gigabyte copy. On
# 2026-08-24 that left 285 source paths on one disk, the largest 24 GB
# each, and took 1.8 TB to 100% full; the cleanup reclaimed ~1.6 TB.
# Through the git fetcher the same tree is a few megabytes, because it
# sees tracked files only.
#
# That is not left to good intentions: the script measures the source
# path it just fetched and refuses to build if it is over the limit
# below. If you are staring at that error, something reintroduced a
# whole-directory copy -- do not raise the limit, find it.
#
# The flip side of "tracked files only" is that a NEW file is invisible
# to the fetcher until it is at least `git add -N`'d. If a case or a
# change you just made appears to have no effect, check that first.
set -uo pipefail

here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel) || {
  echo "not inside a git checkout; the matrix is fetched with git+file:" >&2
  exit 1
}
flakeref="git+file://$root"

# Generous: the tracked tree is single-digit megabytes today, and the
# failure this guards against is three orders of magnitude out.
max_source_mb=${MATRIX_MAX_SOURCE_MB:-512}

filter=${1:-.}

# One evaluation for the whole run: fetch the tree, build pkgs, walk the
# matrix, and emit `<name> <drvPath>` lines plus the source path. Doing
# this per case would re-fetch and re-evaluate 21 times.
read -r -d '' expr <<NIX_EOF
let
  flake = builtins.getFlake "$flakeref";
  matrix = import (flake.outPath + "/home/common/pkgs/cp-eras-ui/tests/matrix.nix") {
    pkgs = flake.out.pkgs;
  };
in
{
  source = flake.outPath;
  cases = matrix.cases;
}
NIX_EOF

apply='m:
  builtins.concatStringsSep "\n" (
    [ ("source " + m.source) ]
    ++ map (n: n + " " + (builtins.getAttr n m.cases).drvPath) (builtins.attrNames m.cases)
  )'

echo "evaluating $flakeref ..."
if ! plan=$(nix eval --impure --raw --expr "$expr" --apply "$apply" 2>/tmp/matrix-eval.$$); then
  echo "evaluation failed:" >&2
  cat /tmp/matrix-eval.$$ >&2
  rm -f /tmp/matrix-eval.$$
  exit 1
fi
rm -f /tmp/matrix-eval.$$

source_path=${plan%%$'\n'*}
source_path=${source_path#source }
source_mb=$(du -sm "$source_path" | cut -f1)
echo "source:     $source_path (${source_mb} MB)"
if [ "$source_mb" -gt "$max_source_mb" ]; then
  echo >&2
  echo "REFUSING TO BUILD: the fetched source is ${source_mb} MB, over the" >&2
  echo "${max_source_mb} MB limit. A tracked checkout of this repo is a few MB." >&2
  echo "Something is copying an untracked directory -- most likely a 'path:'" >&2
  echo "fetcher picking up cargo target/. Read the comment at the top of this" >&2
  echo "script; do not raise the limit." >&2
  exit 1
fi

names=()
drvs=()
while read -r name drv; do
  [ "$name" = "source" ] && continue
  [[ $name =~ $filter ]] || continue
  names+=("$name")
  drvs+=("$drv")
done <<<"$plan"

total=${#names[@]}
if [ "$total" -eq 0 ]; then
  echo "no cases match /$filter/" >&2
  exit 1
fi

logdir=$(mktemp -d -t cp-eras-ui-matrix.XXXXXX)
echo "logs:       $logdir"
echo "cases:      $total"
echo

# Sequential on purpose. Each case gives its render a fixed 15 s to
# settle (tests/visual.nix, `settle`) before the screenshooter fires,
# and that budget is what makes a run flaky under load -- building four
# headless compositors at once would make the thing this script reports
# on worse. One retry covers the occasional loss anyway: two spurious
# failures were seen in one night, both green on the second attempt.
declare -a results
failed=0
i=0
for idx in "${!names[@]}"; do
  name=${names[$idx]}
  drv=${drvs[$idx]}
  i=$((i + 1))
  printf '[%2d/%2d] %-22s ' "$i" "$total" "$name"

  start=$SECONDS
  status=FAIL
  note=""
  for attempt in 1 2; do
    if nix build --no-link --print-build-logs "${drv}^*" \
      >"$logdir/$name.attempt$attempt.log" 2>&1; then
      status=PASS
      [ "$attempt" -eq 2 ] && note=" (retry)"
      break
    fi
    # A failed derivation is not cached, so a retry genuinely re-runs.
    [ "$attempt" -eq 1 ] && printf 'retrying... '
  done
  elapsed=$((SECONDS - start))

  printf '%s%s  %ds\n' "$status" "$note" "$elapsed"
  results+=("$(printf '%-22s %s%s' "$name" "$status" "$note")")
  [ "$status" = FAIL ] && failed=$((failed + 1))
done

echo
echo "================================================"
printf '%s\n' "${results[@]}"
echo "================================================"
if [ "$failed" -eq 0 ]; then
  echo "$total/$total passed"
else
  echo "$((total - failed))/$total passed, $failed FAILED"
  echo "logs in $logdir"
fi
exit $((failed > 0))
