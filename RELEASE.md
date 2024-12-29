# Release Checklist

1. Update date for latest `UNRELEASED` version in `CHANGELOG.md`

2. Update `vulkanalia-sys` crate version

3. Update `vulkanalia` crate version

4. Update `vulkanalia-sys` dependency version for `vulkanalia` crate

5. Update `vulkanalia` dependency version for `vulkanalia-vma` crate

6. Update `vulkanalia` versions in tutorial
    - Version used in tutorial crate
    - Version used in development environment chapter
    - Version used for documentation links in preprocessor

7. Push changes to `master` (ideally via pull request)

8. Run `cargo publish` for `vulkanalia-sys` crate

9. Run `cargo publish` for `vulkanalia` crate

10. Create release on GitHub with `CHANGELOG.md` entry

11. ???

12. Profit!

# MSRV Update Checklist

1. Search for the version (e.g., `1.73`) and replace all references

2. Update targeted Rust version for `bindgen` in `ext/vma/build.rs`
