## Y Surreal
Personal control scripts for a surrealdb instance running on a server somewhere

```sh
# adds necessary environment variables
source ../ymap/env.nu

cargo db production|testing kill
cargo db production clean
cargo db production|testing start
cargo db production import
cargo db production connect
```