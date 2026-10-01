---
id: ban-alter-generated-expression
title: ban-alter-generated-expression
---

## problem

A stored generated column rejects inserts that supply its value. Dropping its expression changes the column semantics.

```sql
ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;
```

## solution

Update client inserts before adding or changing a generated column.
