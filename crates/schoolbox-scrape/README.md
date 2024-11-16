Start DB:
`surreal start rocksdb://db -A --user root --pass root`

Add the COOKIE constant in cookie.txt in src/ folder by signing into schoolbox and copying the first requests Cookie header value.
`cargo r` to start scraping, assumes you have database already setup
