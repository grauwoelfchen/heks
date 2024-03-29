# Heks

[![pipeline](
https://gitlab.com/grauwoelfchen/heks/badges/trunk/pipeline.svg)](
https://gitlab.com/grauwoelfchen/heks/commits/trunk) [![coverage](
https://gitlab.com/grauwoelfchen/heks/badges/trunk/coverage.svg)](
https://gitlab.com/grauwoelfchen/heks/commits/trunk) [![crate::heks](
https://img.shields.io/crates/v/heks?label=crates&style=flat)](
https://crates.io/crates/heks) [![doc::heks](
https://docs.rs/heks/badge.svg)](https://docs.rs/crate/heks)

A lib/tool exchanges hexadecimal colour input.


## Repositories

This is mainly developed on [grauwoelfchen/heks](
https://gitlab.com/grauwoelfchen/heks) on GitLab.com, but the source code is
hosted also on several following repositories.

Any merge/pull requests or issues on any repository are welcomed.

* https://gitlab.com/grauwoelfchen/heks
* https://github.com/grauwoelfchen/heks
* https://git.sr.ht/~grauwoelfchen/heks

```zsh
# the main branch is "trunk"
% git clone git@gitlab.com:grauwoelfchen/heks.git
% git --no-pager branch -v
* trunk xxxxxxx XXX
```


## Install

```zsh
% cargo install heks
```


## Usage

### REPL

```zsh
% heks
> t
invalid value is given
> 888888
rgb(136, 136, 136)
> #222222
rgb(34, 34, 34)
>
```


## Development

### Verify

```zsh
# check code using all verify:xxx targets
% make verify:all
```

See `make help`.

### Test

```zsh
% make test
```

### CI

Run CI jobs on a docker conatiner (`grauwoelfchen/rust:stable`, Gentoo Linux)
using gitlab-runner. See `.gitlab-ci.yml`.

```zsh
# prepare environment variables for CI via .env.ci
% cp .env.ci.sample .env

# e.g. test (see .gitlab-ci.yml)
% make runner-test
```


## License

`GPL-3.0-or-later`

```text
Heks
Copyright 2021 Yasuhiro Яша Asaka

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
```
