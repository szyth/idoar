### This script dumps all user data and saves in a file.
To be used as a PoC for IDOR vulnerabilities in Pentest Engagements.

---

Build the project with:
`cargo build --release`

Run binary:
```bash
cd target/release

# arguments: URL, Start ID, End ID
./idoar  "https://www.google.com/search?q=" 1 20

```
