## Y Surreal
Useful utilities for implementing control scripts for controlling surrealdb instances.

```sh
# adds necessary environment variables
source ../ymap/env.nu

cargo db production|testing kill
cargo db production clean
cargo db production|testing start
cargo db production import
cargo db production connect
```