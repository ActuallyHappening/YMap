## Y Surreal
Personal control scripts for a surrealdb instance running on a server somewhere

```sh
# adds necessary environment variables
source ../ymap/env.nu

cargo db kill
cargo db clean
cargo db start
cargo db import
cargo db connect
```