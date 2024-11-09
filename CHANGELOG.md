# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2024-11-09

### 🚀 Features

- *(nmide-plugin-manager)* Added name-id
- *(nmide-core)* Implemented utility builder

### 🐛 Bug Fixes

- *(nmide-core)* Corrected path creation

### 🚜 Refactor

- *(nmide-core)* Removing folding
- *(nmide-core)* Corrected state-change logic
- *(nmide-core)* Switch to vitejs from nextjs

### 📚 Documentation

- *(CHANGELOG)* Updated changelog
- *(nmide-thesis)* Updated notes
- Updated readme for v1.0.0

### 🎨 Styling

- *(CHANGELOG)* Using default config for cliff

### 🧪 Testing

- *(nmide-core)* Created tests for RSC

### ⚙️ Miscellaneous Tasks

- Updated pipeline
- Removed merge conflict text
- Corrected job name
- Bump version

## [0.1.0] - 2024-08-15

### 🚀 Features

- *(nmide-core)* Enabled test coverage
- *(nmide-core)* Added chtml
- *(nmide-std-lib)* Added fn from Html to CHtml
- *(nmide-cli)* Added cmd system
- *(nmide-core)* Migrated to Tauri v2
- *(nmide-std-lib)* Implemented R* to T*
- *(nmide-core)* Created script-tag for JS plugin
- *(nmide-core)* Created Init function
- *(nmide-core)* Started on JSP-System
- *(nmide-std-lib)* Added merge_mut to rmap
- *(nmide-core)* Started on std JSP library

### 🐛 Bug Fixes

- *(nmide-docker)* Removed unused dependencies
- *(nmide-plugin)* Added all plugins to workspace
- *(nmide-core)* Added omitted type
- *(nmide-core)* Corrected frontendUrl path
- *(nmide-core)* Implemented new ts types
- *(nmide-plugin-manager)* Updated usage of NSL
- *(nmide-core)* Improved Init.tsx
- *(nmide-core)* Corrected Init.tsx
- *(nmide-std-lib)* Updated lib for ease of use
- *(nmide-core)* Updated permissions
- *(nmide-std-lib)* Corrected map methods
- *(nmide-core)* Added conditional-wait to init
- *(nmide-core)* Updated DHtml

### 🚜 Refactor

- *(nmide-plugin)* Removed deprecated exs.
- *(nmide-core)* Split main into components
- *(nmide-core)* Removed debugging code
- *(nmide-core)* Removed msg-handler
- Changed unwrap to unwrap_or_default
- *(nmide-core)* Corrected Tauri sec-conf
- *(nmide-core)* Removed state from backend

### 📚 Documentation

- Updated README.md
- *(nmide-thesis)* Updated daily notes
- *(nmide-thesis)* Updated daily notes
- *(nmide-thesis)* Updated daily notes
- *(nmide-thesis)* Updated daily notes
- *(nmide-thesis)* Updated daily notes
- *(nmide-thesis)* Presentation, what's an IDE?
- *(nmide-thesis)* Daily note update
- *(nmide-thesis)* Updated daily notes
- *(README)* Updated readme
- *(nmide-thesis)* Daily note update
- *(nmide-thesis)* Updated notes

### 🧪 Testing

- *(nmide-core)* Corrected tests
- *(nmide-std-lib)* Added Map parametarized tests

### ⚙️ Miscellaneous Tasks

- *(justfile)* Cleaned up messy justfile #35
- Improved ci: #48
- C-pipelines can now fail
- Added rules to build jobs
- Added nmide-core coverage report
- Corrected image for nmide-coverage
- Changed name from unit test to test
- Corrected image for wrapper job
- Corrected images for jobs
- Refactor gitlab-ci
- Added test-build for nmide-core
- Added release stage
- Corrected dependencies
- Corrected job script
- Added grcov to job
- Added tags
- *(nmide-cli)* Init commit
- *(nmide-core)* Upgraded tauri-plugin versions

## [0.2.0] - 2024-08-31

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
- *(nmide-core)* Added Ord, Eq in Value and Map
- *(nmide-macros)* Added cast_html method to Html
- *(nmide-std-lib)* Added ts-auto-gen for Html
- *(nmide-ide)* Created simple showcase plugin
- *(nmide-core)* Added open file dialog
- *(nmide-plugin)* Can now open/close files
- *(nmide-plugin)* Added some basic css
- *(nmide-core)* Added dev plugin test support

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
- *(nmide-plugin)* Corrected update fn
- *(nmide-wrapper)* Corrected build.rs
- *(nmide-wrapper)* Corrected build.rs
- *(nmide-wrapper)* Corrected build.rs
- *(nmide-lib)* Added new CHtmlElement types
- *(nmide-plugin)* Simplified nmide-manager
- *(nmide-tests)* Updated plugins
- *(nmide-std-lib)* Corrected ts-export for types
- *(nmide-core)* Added P-tag rendering
- *(nmide-plugin)* Corrected Msg
- *(nmide-core)* Corrected RS-TS export path
- *(nmide-core)* Improved Css-Typing

### 💼 Other

- *(nmide-rust-ffi)* Attempt to correct building

### 🚜 Refactor

- *(project)* Reorganized the repo
- *(nmide-lib)* Refactored Makefile
- *(nmide-lib)* Now using CMakeLists instead of Make
- Changed Dockerfile.* to *.Dockerfile
- *(.gitlab-ci.yml)* Moved from `&&` to multiline script
- *(nmide-lib)* Added c-affix
- *(nmide-lib)* Added c-affix
- Removed cliff
- *(nmide-core)* Removed logging
- *(nmide-core)* Client is now ts
- *(nmide-core)* Removed css

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
- *(nmide-thesis)* Created presentation plan
- *(nmide-thesis)* Created code example
- *(nmide-std-lib)* Corrected code in doc-string
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Updated slides
- *(nmide-thesis)* Updated notes
- *(nmide-thesis)* Last fix to presentation
- *(nmide-thesis)* Updated daily notes
- *(nmide-thesis)* Updated daily notes

### 🧪 Testing

- *(nmide-lib)* Removed vals in cmsg
- *(nmide-core)* Added more DOM tests
- *(nmide-lib)* Added greetings function for testing
- *(nmide-lib)* Removed vals in cmsg
- *(nmide-core)* Added more DOM tests
- *(nmide-lib)* Added greetings function for testing
- *(nmide-plugin-c)* Created C plugin for testing
- *(c-test)* Added integration test for c plugins

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
- Update git
- *(nmide-thesis)* Updated presentations
- Update .gitlab-ci.yml file
- Update .gitlab-ci.yml file
- Re-added cliff
- *(nmide)* Added changelogs
- Updated CHANGELOG.md for v0.2.0

<!-- generated by git-cliff -->
