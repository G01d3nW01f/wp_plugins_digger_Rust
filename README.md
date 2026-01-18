# wp-plugin-enumerator

`wp-plugin-enumerator` is a **Rust-based command-line tool** that enumerates **publicly visible WordPress plugins** by analyzing HTML responses and inferring plugin versions from `readme.txt` files.

This tool is intended for **defensive security assessments**, **asset discovery**, and **internal audits** of WordPress sites you own or manage.

---

## 🔍 Purpose

In WordPress environments, especially those managed at scale, it is often necessary to:

- Identify installed plugins
- Detect unreported or forgotten plugins
- Check exposed plugin versions for risk assessment
- Validate submitted plugin inventories

This tool automates that process using **only publicly accessible resources**.

---

## ⚠️ Legal & Ethical Notice

- This tool uses **publicly accessible HTTP resources only**
- No authentication or exploitation is performed
- Use **only on systems you own or have explicit permission to test**
- Unauthorized scanning of third-party systems may be illegal

---

## ✨ Features

- Enumerates plugins via `/wp-content/plugins/<plugin-name>/` references
- Extracts plugin names from page HTML
- Attempts to access each plugin’s `readme.txt`
- Infers plugin version from `readme.txt` ChangeLog section
- Automatic duplicate removal
- Sorted, readable output
- Simple single-argument interface

---

## 📦 Requirements

- Rust (edition 2021 recommended)
- Cargo

### Dependencies
- `reqwest` (blocking)
- `regex`
- `url`

---

## 🚀 Installation

```bash
git clone https://github.com/your-org/wp-plugin-enumerator.git
cd wp-plugin-enumerator
cargo build --release
```

## usage
```
wp-plugin-enumerator <targetURL>

```
## example
```
wp-plugin-enumerator https://example.com

```
## Example Output
```
https://example.com/wp-content/plugins/contact-form-7/
  └ Plugin: contact-form-7
[+] Try Access: https://example.com/wp-content/plugins/contact-form-7/readme.txt
  └ Version: 5.9.3

https://example.com/wp-content/plugins/woocommerce/
  └ Plugin: woocommerce
[+] Try Access: https://example.com/wp-content/plugins/woocommerce/readme.txt
  └ Version: 8.5.1

```
