# http-diff

Extremely basic CLI tool to check a URL for changes.

```bash
# Monitor icanhazip.com for changes every second
$ http-diff --interval 1000 https://icanhazip.com
Initial value is <ip_address_elided>
After 1000ms, there's been no change.
After 2000ms, there's been no change.
After 3000ms, there's been no change.
(...)
```
