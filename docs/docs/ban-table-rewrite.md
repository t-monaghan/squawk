---
id: ban-table-rewrite
title: ban-table-rewrite
---

## problem

Table rewrite operations can hold an `ACCESS EXCLUSIVE` lock and block reads and writes.

```sql
ALTER TABLE t SET LOGGED;
```

## solution

Plan a maintenance window or use a migration that avoids a table rewrite.
