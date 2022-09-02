patchtest-rs
============

Lint patches in `mbox` format, apply patches to repositories and print a report:

```
❯ cargo run -- https://github.com/HerrMuellerluedenscheid/snitch asdfasdf tests/files/0001-test-patch.patch
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/patchtest-rs 'https://github.com/HerrMuellerluedenscheid/snitch' asdfasdf tests/files/0001-test-patch.patch`
net  99% ( 139 kb,   582/  583)  /  idx  43% (  251/  583)  /  chk   0% (   0/   0)
Resolving deltas 331/331
❌ Header field is missing (HeaderFieldError { message: "summary is empty" })
✅ apply patch
```
