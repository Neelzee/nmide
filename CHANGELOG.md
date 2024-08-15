# Changelog

All notable changes to the Nmide-Core project will be documented in this file.

## [0.1.0] - 2024-08-15

### 🚀 Features

- *(justfile)* Added command to build thesis pdf
- *(Dockerfile)* Added possibility to use svn in pipelines
- *(Docker)* Created Images for each job in the pipeline
- *(justfile)* Added cmd to build, tag and push
- *(nmide-lib)* Added munit
- *(nmide-lib)* Added tests
- *(C)* Added c-tests and checks
- *(nmide-lib)* Implemented cmap
- *(nmide-core)* Added testing
- *(justfile)* New build release cmd nmide-lib
- *(.gitlab-ci.yml)* C-check fails on errors
- *(nmide-lib)* Added cmsg
- *(nmide-framework)* Added counter example
- *(nmide-lib)* Added cmsg
- *(nmide-framework)* Added counter example

### 🐛 Bug Fixes

- *(nmide-core)* Corrected usage of ffi-wrapper in nmide-core
- *(justfile)* Corrected just-commands paths
- *(justfile)* Corrected pathing
- *(nmide-thesis)* Corrected invalid json-obj
- *(Dockerfile)* Added missing libraries to tauri-img
- *(justfile)* Added docker build cmd, corrected build-release cmd
- *(nmide-rust-ffi)* Can now call function from C, they just dont work
- *(just)* Corrected cmd, added cmd to push to svn
- *(nmide-rust-ffi)* Fixed issue with building crate
- *(ci)* Corrected pdf-build script
- Corrected pdf-build job
- Pdf-build fix
- *(Docker)* Added js and rs test images
- *(nmide-rust-ffi)* Fixed issue with building
- *(justfile)* Corrected cmd
- *(c.Dockerfile)* Added cmake
- *(.gitlab-ci)* Corrected build script on c-test
- *(.gitlab-ci)* Corrected path in jobs, added pwd
- *(.gitlab-ci)* Corrected invalid git-clone path
- *(nmide-framework)* Corrected pathing to wrapper
- *(.gitlab-ci.yml)* Corrected exec name
- *(RenderHtml)* Added key-props to rendering
- *(Nmide-Lib)* Corrected munit integration
- *(c.Dockerfile)* Added libc
- *(nmide-lib)* Corrected CMAKE_C_STANDARD
- *(nmide-lib)* Corrected source-files for library
- *(nmide-lib)* Added casting to free
- *(.gitlab-ci.yml)* Corrected rule
- *(nmide-core)* Updated wrapper imports
- *(nmide-lib)* Added casting to free
- *(.gitlab-ci.yml)* Corrected rule
- *(nmide-core)* Updated wrapper imports

### 🚜 Refactor

- *(project)* Reorganized the repo
- *(nmide-lib)* Refactored Makefile
- *(nmide-lib)* Now using CMakeLists instead of Make
- Changed Dockerfile.* to *.Dockerfile
- *(.gitlab-ci.yml)* Moved from `&&` to multiline script
- *(nmide-lib)* Added c-affix
- *(nmide-lib)* Added c-affix

### 📚 Documentation

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


