---
id: ban-alter-sequence
title: ban-alter-sequence
---

## problem

Changing a sequence range, type, increment, or current value changes generated values. This rule is opt-in.

```sql
ALTER SEQUENCE s RESTART WITH 10;
```

## solution

Check clients that depend on generated values before changing the sequence.

Enable this rule with `--include ban-alter-sequence` (or add `ban-alter-sequence` to your configured include list).
