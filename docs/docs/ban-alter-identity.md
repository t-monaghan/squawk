---
id: ban-alter-identity
title: ban-alter-identity
---

## problem

Changing identity can make inserts with explicit or omitted values fail.

```sql
ALTER TABLE t ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY;
```

## solution

Update client inserts before changing the identity setting.
