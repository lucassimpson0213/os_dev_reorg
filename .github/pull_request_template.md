## PR Title Format (REQUIRED)

Use Conventional format:

type(scope): description

Examples:
- feat(mm): implement physical page allocator
- fix(boot): correct GDT pointer
- refactor(idt): separate descriptor builder
- chore(ci): add kernel boot check
- docs(paging): document higher-half mapping

Allowed types: feat, fix, refactor, chore, docs

---

## Linked Issue
Closes #

---

## What does this change do?
(Briefly describe the behavior change)

---

## Why is this change needed?
(What problem were you solving? What was broken/confusing?)

---

## How was it tested?
- [ ] Builds successfully
- [ ] Boots in QEMU
- [ ] Boot success marker observed

Notes:

---

## Affected subsystem
- [ ] Boot
- [ ] Memory
- [ ] Interrupts
- [ ] Drivers
- [ ] Scheduler
- [ ] Filesystem
- [ ] CI / Tooling
- [ ] Documentation

---

## Risk / Follow-ups
Anything incomplete, fragile, or planned next?
