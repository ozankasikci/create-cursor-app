## Project: create-cursor-app

### AI Assistant Check-in Protocol
1. At the start of each conversation, execute:
```
You are working on create-cursor-app
Read CHANGELOG.md and PROJECT_SCOPE.md now, report your findings, and follow all instructions.
Begin check-in process and document analysis.
```

### Documentation Maintenance Rules

#### Pre-Action Requirements
1. Read and analyze both CHANGELOG.md and PROJECT_SCOPE.md
2. Report findings: `Read [filename]: [key points for current task]`
3. Review existing context (features, issues, architecture)
4. Proceed only after context verification

#### Post-Change Requirements
1. Update documentation immediately:
   - Add to [Unreleased] in CHANGELOG.md
   - Update PROJECT_SCOPE.md if architecture/features change
2. Report updates using:
   ```
   Updated CHANGELOG.md: [changes]
   Updated PROJECT_SCOPE.md: [changes] (if applicable)
   ```
3. Document all changes with:
   - Feature description
   - Reason for change
   - Implementation details
   - Limitations/considerations

#### Analysis Protocol
When reviewing logs/code:
```
Analyzed [item]: [key findings relevant to task]
```

### [Unreleased]
[Latest changes documented here]
- Added file overwrite confirmation for existing directories
  - Checks if destination directory contains files
  - Prompts user for confirmation before overwriting
  - Skips confirmation in test mode
  - Added test coverage for overwrite scenario
- Added per-file overwrite confirmation
  - Prompts for each existing file individually
  - Allows skipping specific files
  - Preserves unrelated files in target directory
  - Added test coverage for per-file overwrite scenarios
  - Maintains non-conflicting files in destination
- Fixed error handling in file overwrite confirmation
  - Changed error types to properly handle InquireError
  - Improved error propagation in template processing
  - Maintained consistent error handling across all IO operations
- Added GitHub Actions release workflow
  - Automated version bumping
  - Automated release creation
  - Automated crates.io publishing
  - Added release documentation

---
```
Last Updated: [date]
Version: [version]