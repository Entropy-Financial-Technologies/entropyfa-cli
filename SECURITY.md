# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly.

**Do not open a public issue.**

Instead, email **security@entropyfa.com** with:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will acknowledge your report within 48 hours and aim to provide a fix within 7 days for critical issues.

## Scope

entropyfa is a local-only CLI tool with no network access, no file system writes (beyond stdout), and no user data collection. The primary security concerns are:

- **Data accuracy**: Incorrect tax/financial data could lead to wrong decisions. All embedded data is sourced from IRS publications and verified with automated tests.
- **Supply chain**: Binary releases are built via GitHub Actions from tagged commits. Verify checksums before installing.
- **Install script**: The `install.sh` script downloads binaries over HTTPS from GitHub Releases. Review the script before piping to `sh`.

## Disclosure Policy

We follow coordinated disclosure. We will credit reporters in the release notes unless anonymity is requested.
