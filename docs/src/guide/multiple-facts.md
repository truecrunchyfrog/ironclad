# Working with Multiple Facts

Catalogs become interesting when they have several facts with different levels of noise, cost, and dependency.

## Resolve a subset

Resolve only selected facts:

```bash
ic resolve homepage hero-sentence footer-links
```

Resolve everything except a few:

```bash
ic resolve --exclude slow-report --exclude weather-page
```

## Dependencies

If facts import exported values from other facts, Ironclad sorts them so dependencies resolve first.

If a dependency cycle exists, resolution fails.

That usually means two facts are trying to derive each other’s assumptions, which is more poetic than practical.
