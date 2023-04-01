### Parallelized IDOR Exploitation: Multi-Threaded POC Script for Fast Results

This script uses multiple threads to send parallel requests to IDOR-vulnerable endpoints within a specified range. The responses obtained from these requests are then saved to a file. This approach enables an extremely fast execution of the IDOR Proof of Concept (POC) script.

---

Build the project with:
`cargo build --release`

Run binary:
```bash
cd target/release

# arguments: URL, Start ID, End ID
./idoar  "https://www.google.com/search?q=" 1 20

```
