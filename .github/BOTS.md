# Release Workflow

## Trigger

A PR merged into `main`. Closing a PR without merging does nothing, and runs are queued one at a time.

## PR title prefix bump

Prefixes are case-insensitive. No prefix means no release.

| Prefix | Bump | Example |
|---|---|---|
| `[PATCH]`, `[FIX]` | patch | `0.1.0` → `0.1.1` |
| `[FEAT]` | minor | `0.1.1` → `0.2.0` |
| `[BREAKING]` | minor | `0.2.0` → `0.3.0` |
| `[RELEASE]` | major | `0.3.0` → `1.0.0` |

## What the bot does

1. Reads the PR title prefix and picks the bump.
2. Bumps `VERSION`.
3. Lists the PR's commits through the GitHub API (merge commits are skipped).
4. Prepends an entry to `CHANGELOG.md`: `## vX.Y.Z - date`, with commits grouped under Features, Fixes and Other changes.
5. Commits `chore(release): vX.Y.Z`, tags it and pushes to `main`, as the release GitHub App's bot user.
6. Creates the GitHub Release. The body is the PR body plus a link to `CHANGELOG.md`.

## Conventions

- **Commits:** `feat:` goes under Features, `fix:` under Fixes, everything else (`chore:`, `docs:`...) under Other changes.
- **PR title:** prefix only, no version number. The bot computes the number.
- **PR body:** becomes the release notes. Don't add a `CHANGELOG.md` link, the bot appends one.
- **Merge method:** any. The bot reads the PR, not the merge commit.
- **Never edit** `VERSION` or `CHANGELOG.md` by hand.

## Setup

- `VERSION` and `CHANGELOG.md` exist on `main`, and `VERSION` holds the last released version.
- A GitHub App with **Contents: read and write**, installed on the repo.
- The app is on the `main` ruleset's bypass list, mode **Always**.
- Repo secrets `RELEASE_APP_ID` and `RELEASE_APP_KEY` (the `.pem` contents). Never commit the `.pem`.

## Limits

- The GitHub API returns at most 250 commits per PR.
- If a PR has no usable commits, the changelog entry falls back to the PR title.