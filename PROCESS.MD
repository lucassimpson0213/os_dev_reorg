# PROCESS

This document describes the standard development workflow for this
repository. The goal is simple:

> Every change should be traceable from code → commit → pull request →
> issue → discussion.

------------------------------------------------------------------------

## Daily Use (Normal Development)

All normal development should go through **gdd**.\
You typically should **not manually create branches or pull requests**.

### Start Work

Run:

    gdd

You will be prompted to select or create an issue.

`gdd` will automatically: - create a correctly named feature branch -
push the branch - open a pull request linked to the issue

------------------------------------------------------------------------

### Work on the Feature

Edit code normally.

When saving progress:

    git add .
    git commit

A commit helper will prompt you for a Conventional Commit message.

Then push:

    git push

------------------------------------------------------------------------

### Continuous Integration

After pushing: - CI automatically builds the kernel - Boot/tests run
automatically - Status checks appear on the pull request

You do **not** need to manually run CI.

------------------------------------------------------------------------

### Merge

When: - required checks pass - review is satisfied

Auto-merge will merge the PR and close the linked issue.

Because this repository uses **squash merging**, the **PR title becomes
the final commit message**.

------------------------------------------------------------------------

## Workflow Overview

    issue → feature branch → commits → PR → CI → review → squash merge → issue closes

------------------------------------------------------------------------

## Commits

Commit messages follow the **Conventional Commits** format:

    type(scope): message

Examples:

    feat(memory): implement page allocator
    fix(boot): correct grub timeout
    refactor(pmm): simplify free list structure
    docs(paging): document virtual memory layout

Commit messages are generated using the commit helper when running:

    git commit

Do not manually write commit messages unless necessary.

------------------------------------------------------------------------

## Pull Requests

All changes must go through a pull request.

Requirements: - Every PR must reference an issue using
`Closes #<issue number>` - PR title must be a Conventional Commit - PR
template must be completed - Required CI checks must pass before merge

The PR template is located at:

    .github/pull_request_template.md

------------------------------------------------------------------------

## Feature Branch Naming

Feature branches must follow this naming convention:

    feat/123-page-allocator
    fix/77-grub-timeout
    chore/210-ci-cleanup
    docs/54-memory-map-notes
    refactor/88-pmm-structure

Branch names correspond to the issue number.

------------------------------------------------------------------------

## Branch Roles

-   **dev** --- active development integration branch
-   **main** --- stable integration branch
-   **stable** --- release-ready code

Direct pushes are not allowed to these branches.

All work enters through pull requests only.

------------------------------------------------------------------------

## Branch Protections

Organization-wide rules are defined in:

    general-policy.json

Protections include: - no force pushes - required status checks -
required workflows (CI + testing) - CodeFactor quality check

**Merge is blocked unless required status checks pass on the pull
request.**

------------------------------------------------------------------------

## Merge Policy

This project uses **squash merging only** to maintain a clean history.

Auto-merge is enabled when: - all required checks pass - requirements
are satisfied

The PR title becomes the final commit on the target branch.

------------------------------------------------------------------------

## About gdd

`gdd` automates project bookkeeping and setup.

It: - creates or links an issue - creates the correctly named branch -
pushes the branch - opens a pull request with `Closes #<issue>` -
optionally enables auto-merge - prints useful links and next steps

The intent is that contributors spend time writing code, not managing
GitHub paperwork.

------------------------------------------------------------------------

## Why This Exists

Kernel development has very little runtime observability.\
The Git history serves as the primary debugging record.

This workflow ensures: - every change has context - every decision is
recoverable - future debugging is possible - contributors can understand
why code exists

