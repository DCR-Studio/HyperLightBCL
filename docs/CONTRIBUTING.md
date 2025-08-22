# How to Contribute to the HyperLightBCL Project?

## Before You Start

Check the Issues to find tasks you can contribute to, or create a new Issue to discuss your ideas.

---

## Contribution Process

### Reporting Issues

1. Before submitting an Issue, search for existing related Issues
2. Use the provided Issue template
3. Include the following information:
>   - Clear problem description
>   - Reproduction steps (including environment information)
>   - Expected vs. actual behavior
>   - Relevant logs/screenshots (if available)

---

### Submitting Code

1. Fork the repository and clone locally:

```bash
git clone https://github.com/your-username/HyperLightBCL.git
```

2. Create your branch:

```bash
git checkout -b feat/your-feat-name
# or
git checkout -b fix/issue-number-desc
```

3. Follow project code style guidelines
4. Commit changes using Angular commit message convention:

```bash
git commit -m "<type>(<scope>): <subject>"
```

5. Push branch to your Fork:

```bash
git push origin your-branch-name
```

6. Create a Pull Request:
   · Target the upstream repository's dev branch
   · Fill out PR information in detail
   · Link related Issues (if any)

---

## Development Standards

### Testing Requirements

· Ensure code compiles successfully locally before submitting

---

## Angular Commit Convention

### Basic format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Each commit must include the header. Keep subject under 100 characters.

---

### Header

Contains type, optional scope, and subject.

---

### Type

Must be one of: build, chore, ci, docs, feat, fix, perf, refactor, style, test

---

### Scope

Specifies the location of changes

---

### Subject

Succinct description using imperative present tense, no capitalization, no period

---

### Body

Use imperative present tense. Include motivation and contrast with previous behavior.

---

### Footer

### Breaking Changes

Start with BREAKING CHANGE: followed by description and migration notes

---

### Referencing Issues

Use Closes keyword:

```
Closes #1145
```

Multiple issues: Closes #114, #514, #1919

---

### Revert

Header starts with revert:, body includes This reverts commit <hash>.

---

## License

Your contributed code will be licensed under this project's license.
