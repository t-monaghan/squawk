---
id: ban-drop-constraint
title: ban-drop-constraint
---

## problem

Dropping a constraint removes a foreign key, check, or uniqueness guarantee that clients can depend on.

```sql
ALTER TABLE t DROP CONSTRAINT IF EXISTS c;
```

## solution

Update clients to not depend on the constraint before dropping it.
