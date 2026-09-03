# gitpixel

Draw ASCII text on your GitHub contribution graph using backdated commits.

GitPixel turns your contribution graph into a canvas: it renders a word or
phrase across the weeks of a calendar year and generates backdated, empty
commits at the exact grid positions, so the text appears as colored cells on
your GitHub profile.

## Install

```sh
cargo install gitpixel
```

## Usage

GitPixel writes commits into a git repository you already have set up. Point
`--repo` at that repo; it will open it (it never initializes one for you) and
commit using the `user.name` / `user.email` already configured there.

```sh
# Preview only (no commits written, dry run)
gitpixel draw --text "SADIQU" --year 2026 --repo ../my-repo

# Actually write the backdated commits
gitpixel draw --text "SADIQU" --year 2026 --repo ../my-repo --confirm
```

Arguments:

- `--text <TEXT>` — the string to render (required)
- `--year <YEAR>` — which calendar year's grid to draw on (default: current year)
- `--start-week <N>` — which week column to start at (default: 10)
- `--repo <PATH>` — path to the git repo to write into (required for commits)
- `--confirm` — write the commits; without it, only the terminal preview is shown

### Undoing a run

Every successful `draw --confirm` is recorded in a local history log at
`~/.config/gitpixel/history.json`. You can inspect and undo runs cleanly:

```sh
gitpixel history                  # list recorded runs
gitpixel revert                   # undo the most recent run
gitpixel revert --id <ID>         # undo a specific run by id
```

`revert` refuses to run if new commits have been added on top of the recorded
state, so your own work is never clobbered.

## How it works

1. Your text is converted to a 5x7 bitmap font and scaled onto the year grid
   (weeks x days, Sunday-first, matching GitHub's layout).
2. Each lit cell is mapped to a calendar date; padding cells outside the year
   are skipped.
3. `gitpixel draw --confirm` creates one backdated, empty commit per intensity
   level, all stamped at noon UTC, so history stays linear and lands exactly
   where the graph expects it.

## License

MIT