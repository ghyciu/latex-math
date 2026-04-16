All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Upcoming]

### Added
- Added a CHANGELOG file.

### Changed
- Updated the README file to include the Changelog, Documentation, and Repository links.

### Fixed
- Passing equations with LaTeX math mode delimiters (i.e. `$$...$$`) now properly parses the equation instead of counting the delimiters as part of the equation.

## [0.1.2] - 2021-04-15

### Added
- Initial release of the library and command-line tool.
- Added support for parsing LaTeX-style math expressions.
- Added basic functionality to display the list of tokens and the abstract syntax tree (AST) breakdown of each equation.

---

[Unreleased]: https://github.com/ghyciu/latex-math/compare/v0.1.2...v0.2.0-beta
[0.1.2]: https://github.com/ghyciu/latex-math/releases/tag/v0.1.2