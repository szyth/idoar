### A fast and lightweight IDOR PoC Script

---

Build the project with:
`cargo build --release`

Run binary:
```bash
cd target/release

# arguments: URL, Start ID, End ID
./idoar  "https://www.google.com/search?q=" 1 20

```