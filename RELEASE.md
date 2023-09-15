# Release Checklist

1. Update date for latest `UNRELEASED` version in `CHANGELOG.md`

2. Update `vulkanalia-sys` crate version

3. Update `vulkanalia` crate version

4. Update `vulkanalia-sys` dependency version for `vulkanalia` crate

5. Push changes to `master` (ideally via pull request)

6. Run `cargo publish` for `vulkanalia-sys` crate

7. Run `cargo publish` for `vulkanalia` crate

8. Create release on GitHub with `CHANGELOG.md` entry

9. ???

10. Profit!
