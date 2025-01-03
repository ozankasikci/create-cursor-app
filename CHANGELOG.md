## Project: {{project_name}}

### AI Assistant Check-in Protocol
1. At the start of each conversation, execute:
```
You are working on {{project_name}}
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

---
```
Last Updated: [date]
Version: [version]