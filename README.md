![tests](https://github.com/github/docs/actions/workflows/test.yml/badge.svg)

```
,__,
(O,O)  SNOWY
( _/
/_"
```

A CLI for managing packaging and releasing of a git repo.

## Commands:

### release:

    - bumping the semver
    - updating nested packages to reflect the new version
    - commiting repo with git tag as long as git stage is empty.

### deploy:

    - updating a specific tag/commit with a tag (dev/stage/prod) for pickup of a CI Pipeline.
