# Release Checklist

1. Update date for latest `UNRELEASED` version in `CHANGELOG.md`

2. Update `vulkanalia-sys` crate version

3. Update `vulkanalia` crate version

4. Update `vulkanalia-sys` dependency version for `vulkanalia` crate

5. Update `vulkanalia` versions in tutorial
    - Version used in tutorial crate
    - Version used in development environment chapter
    - Version used for documentation links in preprocessor

6. Push changes to `master` (ideally via pull request)

7. Run `cargo publish` for `vulkanalia-sys` crate

8. Run `cargo publish` for `vulkanalia` crate

9. Create release on GitHub with `CHANGELOG.md` entry

10. ???

11. Profit!
