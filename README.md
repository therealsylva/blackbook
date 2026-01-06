
# blackbook

A high-performance, asynchronous OSINT tool designed for advanced profile reconnaissance on Instagram. It scrapes candidate lists, queries API endpoints, and correlates data points (Name, Email, Phone) to identify target profiles with precision.

## Features

-   **High Performance:** Built with Rust and Tokio for asynchronous, non-blocking I/O. Significantly faster than synchronous scripting languages.
-   **Smart Rate Limiting:** Implements a Governor (Token Bucket) algorithm to respect API limits and handle 429 responses with exponential backoff.
-   **Secure Credential Management:** Session IDs are handled via environment variables (`.env`) instead of CLI flags to prevent shell history leaks.
-   **Robust Validation:** Validates input formats (Email, Phone) and checks session validity before execution to prevent partial failures.
-   **Machine-Readable Output:** Supports standard text output for human analysis and JSON output for integration with pipelines (e.g., jq, Elasticsearch).
-   **Modular Architecture:** Clean separation of concerns (Client, Scraper, Logic, Output) for maintainability.

## Installation

### Prerequisites
-   Rust (1.70 or later) and Cargo.

### Build from Source

```bash
git clone https://github.com/yourusername/blackbook.git
cd blackbook
cargo build --release
```

The binary will be located at `target/release/blackbook`.

## Configuration

`blackbook` requires an active Instagram Session ID to access internal API endpoints. For security, this must be provided as an environment variable.

1.  Create a `.env` file in the project directory (or export it in your shell):
    ```bash
    SESSION_ID=your_session_id_here
    ```

2.  How to get your Session ID:
    *   Log in to Instagram via a web browser.
    *   Open Developer Tools (F12) -> Application -> Cookies.
    *   Find the cookie named `sessionid` and copy its value.

## Usage

### Basic Usage

Search for a target by name, email, and phone. The tool will scrape potential candidates and output matching details.

```bash
./blackbook -n "John Doe" -e "john.doe@example.com" -p "+15550199"
```

### JSON Output

For piping data into other tools or databases, use the `--json` flag.

```bash
./blackbook -n "John Doe" -e "john.doe@example.com" -p "+15550199" --json
```

### Rate Limiting & Timeout

You can enforce a custom timeout between requests using the `-t` flag (seconds).

```bash
./blackbook -n "John Doe" -e "john.doe@example.com" -p "+15550199" -t 2
```

### Help

```bash
./blackbook --help
```

## Match Levels

The tool calculates a confidence score based on the provided intelligence:

-   **HIGH:** Name, Email, and Phone match.
-   **MEDIUM:** Two out of three metrics match.
-   **LOW:** One out of three metrics match.

## Disclaimer

This tool is for educational purposes and authorized security testing only. The developers are not responsible for any misuse of this software. Ensure you have permission to scan the targets you are investigating.

## Credits

Original Python implementation concept: `yesitsme`  
Rust Rewrite & Architecture: **blackeko5**
```
