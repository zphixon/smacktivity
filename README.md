# smacktivity

(eventually) a single-user activitypub implementation and activitypub library
for rust


## TODO

- get in spec
- serde
    - parsing in a nice way that gracefully handles broken stuff
- networking (dereferencing JSON-LD links)
    - how is serde gonna work?

```rust
// im imagining an api sorta like this
reqwest::get(url)
    .await?
    .json::<Object>()
    .await?
    .resolve() // maybe choose what network provider here?
    .await?;
```


## parts that are currently out-of-spec

- @context
- a bunch of property types
    - closer now but still some stuff like datetime, language tags, duration,
      units, rel, @context need to be corrected
- object hierarchy
    - probably won't do this one. doesn't seem super useful for implementors.
