# Fahrradturm

## Database Model:

```ts
{
  towers: Collection[{
    location: Timestamp,
    name: String,
    boxes: Collection[{
      level: Number,
      index: Number,
      type: strEnum("biclycle", "storage"),
      isPowered: Boolean,
    }],
  }],
  users: Collection[{
    rentals: Collection[{
      start: Timestamp || null,
      end: Timestamp || null,
      tags: Map{
        box: Boolean,
        user: Boolean,
      },
      box: Reference("towers/<id>/boxes/<id>"),
      keys: Collection[{
          enabled: Boolean
          start: Timestamp || null,
          end: Timestamp || null,
          storeCount: Number || null,
          retrieveCount: Number || null,
      }],
    }],
  }],
}
```
