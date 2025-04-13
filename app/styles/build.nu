# In nushell, to watch
# trunk serve
# watch . --glob=**/*.rs {|| nu styles/build.nu}

RUST_LOG=info stylers --output-path /home/ah/Desktop/YMap/app/styles/stylers.css --search-dir /home/ah/Desktop/YMap/app/src
