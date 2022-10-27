### How to reproduce
```shell
export CARGO_REGISTRIES_FLUENCE_TOKEN=yMHU2kPlaic5A9KfzU8v0dPuHJ5qjbgv

cd user_of_user_of_api/
cargo check
```

### Error
```
% cargo check
    Updating `fluence` index
    Updating crates.io index
  Downloaded user_of_that_api v0.1.0 (registry `fluence`)
  Downloaded 1 crate (771 B) in 0.28s
    Checking api_on_crates_io v0.1.0
    Checking user_of_that_api v0.1.0 (registry `fluence`)
error[E0433]: failed to resolve: use of undeclared crate or module `api`
 --> /Users/folex/.cargo/registry/src/crates.fluence.dev-4cf026a00c52b3cc/user_of_that_api-0.1.0/src/lib.rs:1:5
  |
1 | use api::api::*;
  |     ^^^ use of undeclared crate or module `api`

error[E0422]: cannot find struct, variant or union type `Export` in this scope
 --> /Users/folex/.cargo/registry/src/crates.fluence.dev-4cf026a00c52b3cc/user_of_that_api-0.1.0/src/lib.rs:4:13
  |
4 |     let _ = Export {
  |             ^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 | use api_on_crates_io::api::Export;
```