---
id: ban-refresh-materialized-view-non-concurrent
title: ban-refresh-materialized-view-non-concurrent
---

## problem

A non-concurrent materialized view refresh blocks queries that read the view. This rule is opt-in.

```sql
REFRESH MATERIALIZED VIEW mv;
```

## solution

Use `REFRESH MATERIALIZED VIEW CONCURRENTLY` with a suitable unique index.

Enable this rule with `--include ban-refresh-materialized-view-non-concurrent` (or add `ban-refresh-materialized-view-non-concurrent` to your configured include list).
