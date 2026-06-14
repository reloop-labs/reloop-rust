# Contributing to the Reloop Rust SDK

Crates.io package: **`reloop`**.

**License:** [Apache License 2.0](./LICENSE) with additional use restrictions from Reloop Labs.

**API reference:** [reloop.sh/docs](https://reloop.sh/docs)

Port new endpoints from the [Node.js SDK](https://github.com/reloop-labs/reloop-node) reference.

---

## Development setup

```bash
git clone git@github.com:reloop-labs/reloop-rust.git
cd reloop-rust
cargo test
```

Requires **Rust stable**.

---

## Project layout

```
src/
  client.rs
  models.rs
  services/           # mail.rs, domain.rs, …
Cargo.toml              # version, license
```

---

## Conventions

| Topic | Rule |
|-------|------|
| Domain params | Structs with snake_case `serde` fields |
| Mail send | `serde_json::Value` or typed structs |
| Responses | `#[serde(rename_all = "camelCase")]` where API returns camelCase |
| Tests | Route unit tests in service modules |

---

## Pull request checklist

- [ ] `cargo test` passes
- [ ] `Cargo.toml` + `Cargo.lock` (reloop entry) updated on release

---

## Releasing

Version: **`Cargo.toml`** → `version`.

```bash
git commit -am "chore: release v1.9.0"
git push origin main
git tag v1.9.0
git push origin v1.9.0
```

[`.github/workflows/release.yml`](./.github/workflows/release.yml) creates a GitHub Release with source zip.

Publish: `cargo publish` via [`.github/workflows/publish.yml`](./.github/workflows/publish.yml).
