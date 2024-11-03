# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project tries to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-08-31

### Added

- Added cast_html method to Html
- Added ts-auto-gen for Html
- Added new CHtmlElement types
- Added integration test for c plugins
- Added P-tag rendering
- Added open file dialog
- Added dev plugin test support
- Added changelogs

### Changed

- Created presentation plan
- Created code example
- Update git
- Updated presentations
- Added Ord, Eq in Value and Map
- Corrected code in doc-string
- Update .gitlab-ci.yml file
- Update .gitlab-ci.yml file
- Updated notes
- Created simple showcase plugin
- Client is now ts
- Updated slides
- Updated notes
- Can now open/close files
- Added some basic css
- Last fix to presentation
- Updated daily notes
- Updated daily notes
- Re-added cliff

### Fixed

- Corrected update fn
- Corrected build.rs
- Corrected build.rs
- Corrected build.rs
- Simplified nmide-manager
- Created C plugin for testing
- Updated plugins
- Corrected ts-export for types
- Corrected Msg
- Corrected RS-TS export path
- Improved Css-Typing

### Removed

- Removed cliff
- Removed logging
- Removed css

## [0.1.0] - 2024-08-15

### Added

- Added gitignore file
- Added thesis
- Added command to build thesis pdf
- Added job to build pdf
- Added just and pdflatex to img
- Added missing libraries to tauri-img
- Added docker build cmd, corrected build-release cmd
- Added unrelevant CMakeFiles to gitignore
- Added cmd to build, tag and push
- Added js and rs test images
- Added munit
- Added tests
- Added images for testing
- Added static code analysis
- Added init cmd to build-release
- Added c-tests and checks
- Added cmake
- Added key-props to rendering
- Added testing
- Added install to js-test
- Added libc
- Added notes
- Added cmsg
- Added more DOM tests
- Added docs for code-gen
- Added new notes
- Added c-affix
- Added more notes
- Added greetings function for testing
- Added casting to free
- Added $ENABLED to disable jobs
- Added counter example
- Added notes
- Added cmsg
- Added more DOM tests
- Added docs for code-gen
- Added new notes
- Added c-affix
- Added more notes
- Added greetings function for testing
- Added casting to free
- Added $ENABLED to disable jobs
- Added counter example

### Changed

- Updated README.md
- Added CHANGELOG.md
- Reorganized the repo
- Attempt to correct building
- Refactored Makefile
- Refactored pipelines
- Moved from overleaf
- Corrected pdf-job
- Updated svn
- Now using CMakeLists instead of Make
- Added possibility to use svn in pipelines
- Created Images for each job in the pipeline
- Change artifact directory
- Only builds pdf on change
- Changed Dockerfile.* to *.Dockerfile
- V0.1.0 release
- Updated readme to nmide-lib
- Implemented cmap
- Moved from `&&` to multiline script
- New build release cmd nmide-lib
- Corrected tests
- Corrected tests
- C-check fails on errors
- Bugfixing
- Corrected report filename
- Allows failure on c-check job
- Corrected dependency name
- Corrected c-* jobs
- Bugfix
- Bugfix
- Bugfix
- Bugfid
- Corrected exit code
- Corrected correction of exit code
- Bugfix
- Bugfix?
- Bugfix.
- Corrected report path
- Updated notes
- Updated notes
- Corrected rules
- Corrected job
- Updated notes
- Updated notes
- Corrected dependency name
- Corrected c-* jobs
- Bugfix
- Bugfix
- Bugfix
- Bugfid
- Corrected exit code
- Corrected correction of exit code
- Bugfix
- Bugfix?
- Bugfix.
- Corrected report path
- Updated notes
- Updated notes
- Corrected rules
- Corrected job
- Updated notes
- Updated notes
- Corrected job

### Fixed

- Corrected usage of ffi-wrapper in nmide-core
- Corrected just-commands paths
- Corrected pathing
- Corrected invalid json-obj
- Can now call function from C, they just dont work
- Corrected cmd, added cmd to push to svn
- Fixed issue with building crate
- Corrected pdf-build script
- Corrected pdf-build job
- Pdf-build fix
- Fixed issue with building
- Corrected cmd
- Corrected build script on c-test
- Corrected path in jobs, added pwd
- Corrected invalid git-clone path
- Corrected pathing to wrapper
- Corrected exec name
- Corrected munit integration
- Corrected CMAKE_C_STANDARD
- Corrected source-files for library
- Corrected rule
- Updated wrapper imports
- Corrected rule
- Updated wrapper imports

### Removed

- Removed unconventional commits
- Removed linting job
- Removed static code analysis
- Removed vals in cmsg
- Removed vals in cmsg

- *(README)* Updated README.md
- *(README)* Updated README.md
- *(CHANGELOG)* Added CHANGELOG.md
- *(nmide-thesis)* Added thesis
- *(nmide-thesis)* Moved from overleaf
- *(nmide-lib)* Updated readme to nmide-lib
- *(nmide-thesis)* Added notes
- *(nmide-wrapper)* Added docs for code-gen
- *(nmide-thesis)* Added new notes
- *(nmide-thesis)* Added more notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Added notes
- *(nmide-wrapper)* Added docs for code-gen
- *(nmide-thesis)* Added new notes
- *(nmide-thesis)* Added more notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated notes

### 🧪 Testing

- *(nmide-lib)* Removed vals in cmsg
- *(nmide-core)* Added more DOM tests
- *(nmide-lib)* Added greetings function for testing
- *(nmide-lib)* Removed vals in cmsg
- *(nmide-core)* Added more DOM tests
- *(nmide-lib)* Added greetings function for testing

### ⚙️ Miscellaneous Tasks

- *(cliff.toml)* Removed unconventional commits
- *(nmide-rust-ffi)* Added gitignore file
- *(.gitlab-ci.yml)* Refactored pipelines
- Removed linting job
- *(pdf)* Added job to build pdf
- Corrected pdf-job
- *(Dockerfile)* Added just and pdflatex to img
- *(svn)* Updated svn
- *(nmide-lib)* Added unrelevant CMakeFiles to gitignore
- Change artifact directory
- Only builds pdf on change
- *(nmide-lib)* V0.1.0 release
- Added images for testing
- Added static code analysis
- Removed static code analysis
- *(justfile)* Added init cmd to build-release
- Corrected tests
- Corrected tests
- Added install to js-test
- Bugfixing
- Corrected report filename
- Allows failure on c-check job
- *(.gitlab-ci.yml)* Corrected dependency name
- *(.gitlab-ci.yml)* Corrected c-* jobs
- Bugfix
- Bugfix
- Bugfix
- Bugfid
- Corrected exit code
- Corrected correction of exit code
- Bugfix
- Bugfix?
- Bugfix.
- Corrected report path
- *(.gitlab-ci.yml)* Added $ENABLED to disable jobs
- Corrected rules
- Corrected job
- *(.gitlab-ci.yml)* Corrected dependency name
- *(.gitlab-ci.yml)* Corrected c-* jobs
- Bugfix
- Bugfix
- Bugfix
- Bugfid
- Corrected exit code
- Corrected correction of exit code
- Bugfix
- Bugfix?
- Bugfix.
- Corrected report path
- *(.gitlab-ci.yml)* Added $ENABLED to disable jobs
- Corrected rules
- Corrected job
- Corrected job

### Build

- *(nmide-rust-ffi)* Attempt to correct building

