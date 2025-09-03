# Documentation Policy

## Canonical sources
- User-facing docs: GitBook (`https://bert.gitbook.io/bert-documentation`) — source of truth
- Repo `docs/`: developer technical references and active engineering work
- UX stories: `bert/bert/docs/ux/` (non-coding design/user stories)
- Historical materials: `bert/bert/archive/` (not canonical)
- Experimental: `bert/bert/private-dev/` (not canonical)

## What goes where
- Tutorials, user guides, theory → GitBook
- Architecture, technical specs, references → `docs/architecture/` and `docs/technical/`
- UX story authoring → `docs/ux/user-stories/*.md` (indexed in `docs/ux/README.md`)
- Issue references: Always link to file paths, not screenshots; prefer stable permalinks
- "Where to put code" guidance lives in `docs/architecture/QUICK_OVERVIEW.md`

## Contribution rules
- If writing for end users: add/update GitBook; link from repo if needed
- If writing for developers: add/update under `docs/` with concise purpose and links
- Historical/experimental: leave in `archive/` or `private-dev/` with a clear scope note
- PR checklist: link to GitBook/repo docs you updated; no dead links

## Maintenance
- Keep `DOCUMENTATION_GUIDE.md` current as the single entry point
- Prefer updating existing docs over adding new top-level files