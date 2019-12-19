# rust-socks
## Testing
1. Executing 
```bash
cargo run
```
Will report
```
listening on V6([::]:xxxxx)
```
where *xxxxx* is populated with a port number

2. The port *xxxxx* should be supplied to cURL for proxy connection
```bash
curl -x localhost:xxxxx google.co.uk -v
```
## Useful links
1. [wikipedia page on SOCKS](https://en.wikipedia.org/wiki/SOCKS)
2. [SOCKSv5 RFC](https://www.ietf.org/rfc/rfc1928.txt)