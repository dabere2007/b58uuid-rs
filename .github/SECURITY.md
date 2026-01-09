# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take the security of b58uuid-rs seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Please Do Not

- Open a public GitHub issue for security vulnerabilities
- Disclose the vulnerability publicly before it has been addressed

### Please Do

1. **Email us directly** at the repository maintainer's email (found in Cargo.toml or GitHub profile)
2. **Provide detailed information** including:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Suggested fix (if any)
3. **Allow reasonable time** for us to respond and address the issue

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours
- **Updates**: We will send you regular updates about our progress
- **Timeline**: We aim to address critical vulnerabilities within 7 days
- **Credit**: We will credit you in the security advisory (unless you prefer to remain anonymous)

### Security Considerations

This library handles UUID encoding/decoding and includes:

- **No external dependencies**: Reduces attack surface
- **Memory safety**: Rust's memory safety guarantees prevent common vulnerabilities
- **Input validation**: All inputs are validated before processing
- **Overflow protection**: Arithmetic operations include overflow checks
- **Secure random generation**: Uses platform-specific secure random sources

### Known Limitations

- **Windows random generation**: The current Windows implementation uses a fallback method that is not cryptographically secure. This is documented in the code and will be addressed in a future release using BCryptGenRandom.

### Security Best Practices

When using b58uuid-rs:

1. **Keep dependencies updated**: Regularly update to the latest version
2. **Validate inputs**: Always validate UUIDs and Base58 strings from untrusted sources
3. **Handle errors**: Properly handle all error cases
4. **Use secure random**: For UUID generation, ensure your platform has a secure random source

## Security Updates

Security updates will be released as patch versions and announced through:

- GitHub Security Advisories
- Release notes
- Cargo.toml version updates

Thank you for helping keep b58uuid-rs and its users safe!
