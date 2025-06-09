# Code Signing and Distribution

This document explains the security warnings users see when installing fsPrompt and what would be required to eliminate them.

## Current Situation

fsPrompt binaries are **not code-signed**. This means:
- **macOS**: Gatekeeper shows "unidentified developer" warnings
- **Windows**: SmartScreen shows "unrecognized app" warnings
- **Linux**: No warnings (doesn't use code signing for binaries)

## Why No Code Signing?

### Cost

1. **Apple Developer Program**: $99/year
   - Required for macOS code signing
   - Includes notarization service
   
2. **Windows Code Signing Certificate**: $200-500/year
   - Basic OV (Organization Validation): ~$200/year
   - EV (Extended Validation) for instant SmartScreen reputation: ~$500/year

3. **Total Annual Cost**: $300-600/year

### Process Complexity

#### macOS
1. Enroll in Apple Developer Program
2. Create Developer ID certificates
3. Sign the binary: `codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" fsprompt`
4. Create signed installer: `productsign --sign "Developer ID Installer: Your Name" unsigned.pkg signed.pkg`
5. Notarize with Apple: `xcrun altool --notarize-app --primary-bundle-id "com.fsprompt.app"`
6. Staple the notarization: `xcrun stapler staple signed.pkg`

#### Windows
1. Purchase code signing certificate
2. Install certificate in Windows certificate store
3. Sign with signtool: `signtool sign /tr http://timestamp.digicert.com /td sha256 /fd sha256 /a fsprompt.exe`
4. Sign the installer as well
5. Build SmartScreen reputation over time (or pay for EV cert)

## Alternatives for Users

### 1. Install via Package Managers (Future)

Package managers handle signing:
- **macOS**: Homebrew (signs with their certificate)
- **Windows**: Scoop/Chocolatey (community-maintained)
- **Linux**: apt/yum/snap (repository signing)

### 2. Install via Cargo (Recommended)

```bash
cargo install --git https://github.com/patrikpersson/codext-rs.git
```

No security warnings because:
- You're building from source
- Trust is based on source code visibility
- No pre-built binaries involved

### 3. Build from Source

Most secure option - you control the entire build process.

## Future Options

### If fsPrompt Grows

If the project gains significant users, options include:
1. **Sponsorship**: Cover signing costs through GitHub Sponsors
2. **Organization**: Form an open-source organization for shared certificates
3. **Corporate Backing**: Find a company willing to sign releases

### Community Solutions

1. **Reproducible Builds**: Allow users to verify binaries match source
2. **Web of Trust**: Have multiple developers sign releases
3. **Transparency Logs**: Publish build logs and hashes

## For Developers

If you want to sign your own builds:

### macOS (Free for Personal Use)
```bash
# Ad-hoc signing (no Developer ID required)
codesign --deep --force --sign - fsprompt

# This removes "damaged app" errors but still shows "unidentified developer"
```

### Windows (Self-Signed)
```powershell
# Create self-signed certificate (PowerShell as Admin)
New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=Your Name" -CertStoreLocation Cert:\CurrentUser\My

# Sign the executable
signtool sign /a fsprompt.exe
```

Note: Self-signed certificates still trigger warnings but prove the binary hasn't been tampered with.

## Conclusion

Code signing is a "pay-to-play" system designed for commercial software. Open-source projects must choose between:
1. Paying hundreds of dollars annually
2. Living with security warnings
3. Using alternative distribution methods

fsPrompt chooses transparency over payment - our builds are automated, public, and verifiable through GitHub Actions.