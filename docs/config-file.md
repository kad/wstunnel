# Configuration File Support

wstunnel now supports configuration files in multiple formats (YAML, TOML, JSON) as an alternative to command-line arguments. This makes it easier to manage complex configurations and reuse them across deployments.

## Supported Formats

The `config` crate automatically detects the format based on file extension:

- **YAML**: `.yaml` or `.yml` (recommended)
- **TOML**: `.toml`
- **JSON**: `.json`

## Usage

Use the `--config` flag to specify a configuration file:

```bash
# Using YAML format
wstunnel --config config-client.yaml client

# Using TOML format
wstunnel --config config-client.toml client

# Using JSON format
wstunnel --config config-client.json client

# Using a server config
wstunnel --config config-server.yaml server
```

**Note:** CLI arguments take precedence over config file values. This allows you to override specific settings without modifying the config file.

## Configuration File Format

Configuration files can contain either a `client` or `server` section (or both).

### YAML Format (Recommended)

```yaml
client:
  # Local to remote tunnels (required)
  local_to_remote:
    - "tcp://8080:localhost:80"
    - "socks5://127.0.0.1:1080"
  
  # Server address (required)
  remote_addr: "wss://tunnel.example.com:443"
  
  # Connection settings
  connection_min_idle: 5
  connection_retry_max_backoff: "5m"
  
  # TLS settings
  tls_verify_certificate: true
```

### TOML Format

```toml
[client]
local_to_remote = [
    "tcp://8080:localhost:80",
    "socks5://127.0.0.1:1080"
]
remote_addr = "wss://tunnel.example.com:443"
connection_min_idle = 5
connection_retry_max_backoff = "5m"
tls_verify_certificate = true
```

### JSON Format

```json
{
  "client": {
    "local_to_remote": [
      "tcp://8080:localhost:80",
      "socks5://127.0.0.1:1080"
    ],
    "remote_addr": "wss://tunnel.example.com:443",
    "connection_min_idle": 5,
    "connection_retry_max_backoff": "5m",
    "tls_verify_certificate": true
  }
}
```

## Configuration Options

### Client Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `local_to_remote` | list of strings | `[]` | Local to remote tunnel specifications |
| `remote_to_local` | list of strings | `[]` | Remote to local (reverse) tunnel specifications |
| `remote_addr` | string | required | WebSocket/HTTP2 server URL |
| `connection_min_idle` | integer | `0` | Connection pool size |
| `connection_retry_max_backoff` | duration | `"5m"` | Maximum retry backoff time |
| `reverse_tunnel_connection_retry_max_backoff` | duration | `"1s"` | Reverse tunnel retry backoff |
| `tls_sni_override` | string | null | Override SNI for TLS |
| `tls_sni_disable` | boolean | `false` | Disable SNI |
| `tls_ech_enable` | boolean | `false` | Enable encrypted SNI |
| `tls_verify_certificate` | boolean | `false` | Verify server certificates |
| `tls_certificate` | path | null | Client certificate for mTLS |
| `tls_private_key` | path | null | Client private key for mTLS |
| `http_proxy` | string | null | HTTP proxy (format: `user:pass@host:port`) |
| `http_proxy_login` | string | null | HTTP proxy login |
| `http_proxy_password` | string | null | HTTP proxy password |
| `http_upgrade_path_prefix` | string | `"v1"` | HTTP upgrade path prefix |
| `http_upgrade_credentials` | string | null | Basic auth credentials |
| `http_headers` | list of strings | `[]` | Custom HTTP headers |
| `http_headers_file` | path | null | File containing HTTP headers |
| `websocket_ping_frequency` | duration | `"30s"` | WebSocket ping interval |
| `websocket_mask_frame` | boolean | `false` | Enable frame masking |
| `dns_resolver` | list of URLs | `[]` | Custom DNS resolvers |
| `dns_resolver_prefer_ipv4` | boolean | `false` | Prefer IPv4 over IPv6 |
| `socket_so_mark` | integer | null | SO_MARK socket option (Linux only) |

### Server Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `remote_addr` | string | required | Bind address for server |
| `socket_so_mark` | integer | null | SO_MARK socket option (Linux only) |
| `websocket_ping_frequency` | duration | `"30s"` | WebSocket ping interval |
| `websocket_mask_frame` | boolean | `false` | Enable frame masking |
| `dns_resolver` | list of URLs | `[]` | Custom DNS resolvers |
| `dns_resolver_prefer_ipv4` | boolean | `false` | Prefer IPv4 over IPv6 |
| `restrict_to` | list of strings | null | Allowed destinations |
| `restrict_http_upgrade_path_prefix` | list of strings | null | Allowed path prefixes |
| `restrict_config` | path | null | Path to restrictions YAML file |
| `tls_certificate` | path | null | Server certificate |
| `tls_private_key` | path | null | Server private key |
| `tls_client_ca_certs` | path | null | Client CA certificates for mTLS |
| `http_proxy` | string | null | HTTP proxy |
| `http_proxy_login` | string | null | HTTP proxy login |
| `http_proxy_password` | string | null | HTTP proxy password |
| `remote_to_local_server_idle_timeout` | duration | `"3m"` | Reverse tunnel idle timeout |

### Duration Format

Durations can be specified with suffixes:
- `s` for seconds (e.g., `"30s"`)
- `m` for minutes (e.g., `"5m"`)
- `h` for hours (e.g., `"2h"`)

### Tunnel Specification Format

Tunnels are specified as strings in the format:

```
{protocol}://[BIND_ADDR:]PORT:DEST_HOST:DEST_PORT[?options]
```

Examples:
- `"tcp://8080:localhost:80"` - TCP tunnel from local port 8080 to localhost:80
- `"udp://5353:1.1.1.1:53"` - UDP tunnel from local port 5353 to 1.1.1.1:53
- `"socks5://127.0.0.1:1080"` - SOCKS5 proxy on 127.0.0.1:1080
- `"http://127.0.0.1:8080"` - HTTP proxy on 127.0.0.1:8080
- `"tcp://8080:localhost:80?proxy_protocol"` - TCP tunnel with proxy protocol header

## Combining CLI and Config File

You can combine CLI arguments with a config file. CLI arguments always take precedence:

```bash
# Use config file but override the remote address
wstunnel --config my-config.yaml client wss://different-server.com
```

## Example Configurations

### Simple Client (YAML)

```yaml
client:
  local_to_remote:
    - "tcp://3000:localhost:3000"
  remote_addr: "ws://tunnel.example.com:8080"
```

### Simple Client (TOML)

```toml
[client]
local_to_remote = ["tcp://3000:localhost:3000"]
remote_addr = "ws://tunnel.example.com:8080"
```

### Simple Client (JSON)

```json
{
  "client": {
    "local_to_remote": ["tcp://3000:localhost:3000"],
    "remote_addr": "ws://tunnel.example.com:8080"
  }
}
```

### Client with TLS and Custom Headers (YAML)

```yaml
client:
  local_to_remote:
    - "tcp://8080:backend.local:80"
  remote_addr: "wss://tunnel.example.com:443"
  tls_verify_certificate: true
  http_headers:
    - "Authorization: Bearer mytoken123"
    - "X-Custom-Header: value"
  websocket_ping_frequency: "15s"
```

### Client with TLS and Custom Headers (TOML)

```toml
[client]
local_to_remote = ["tcp://8080:backend.local:80"]
remote_addr = "wss://tunnel.example.com:443"
tls_verify_certificate = true
http_headers = [
    "Authorization: Bearer mytoken123",
    "X-Custom-Header: value"
]
websocket_ping_frequency = "15s"
```

### Server with mTLS

```yaml
server:
  remote_addr: "wss://0.0.0.0:8080"
  tls_certificate: "/etc/wstunnel/server-cert.pem"
  tls_private_key: "/etc/wstunnel/server-key.pem"
  tls_client_ca_certs: "/etc/wstunnel/client-ca.pem"
  restrict_config: "/etc/wstunnel/restrictions.yaml"
```

### Multi-Tunnel Client

```yaml
client:
  local_to_remote:
    - "tcp://8080:web.internal:80"
    - "tcp://8443:web.internal:443"
    - "udp://5353:dns.internal:53"
    - "socks5://127.0.0.1:1080"
  remote_addr: "wss://tunnel.example.com:443"
  connection_min_idle: 10
  dns_resolver:
    - "dns://1.1.1.1"
  dns_resolver_prefer_ipv4: true
```

## Tips

1. **Choose Your Format**: Use YAML for readability, TOML for simplicity, or JSON for programmatic generation
2. **Version Control**: Store your config files in version control (excluding sensitive credentials)
3. **Environment-Specific Configs**: Create separate config files for development, staging, and production
4. **Security**: Use file permissions to protect config files containing credentials (e.g., `chmod 600 config.yaml`)
5. **Testing**: Test your configuration with `--log-lvl DEBUG` to see detailed connection information
6. **Validation**: The config file is validated on load - you'll get clear error messages if something is wrong
7. **Format Detection**: The file format is automatically detected from the extension (.yaml/.yml, .toml, .json)

## Migration from CLI Arguments

If you have a complex command line like:

```bash
wstunnel client \
  -L tcp://8080:localhost:80 \
  -L tcp://8443:localhost:443 \
  -L socks5://127.0.0.1:1080 \
  --connection-min-idle 5 \
  --dns-resolver dns://1.1.1.1 \
  wss://tunnel.example.com
```

You can convert it to a config file in your preferred format:

**YAML (config.yaml):**

```yaml
client:
  local_to_remote:
    - "tcp://8080:localhost:80"
    - "tcp://8443:localhost:443"
    - "socks5://127.0.0.1:1080"
  remote_addr: "wss://tunnel.example.com"
  connection_min_idle: 5
  dns_resolver:
    - "dns://1.1.1.1"
```

**TOML (config.toml):**

```toml
[client]
local_to_remote = [
    "tcp://8080:localhost:80",
    "tcp://8443:localhost:443",
    "socks5://127.0.0.1:1080"
]
remote_addr = "wss://tunnel.example.com"
connection_min_idle = 5
dns_resolver = ["dns://1.1.1.1"]
```

**JSON (config.json):**

```json
{
  "client": {
    "local_to_remote": [
      "tcp://8080:localhost:80",
      "tcp://8443:localhost:443",
      "socks5://127.0.0.1:1080"
    ],
    "remote_addr": "wss://tunnel.example.com",
    "connection_min_idle": 5,
    "dns_resolver": ["dns://1.1.1.1"]
  }
}
```

And run it simply as:

```bash
wstunnel --config config.yaml client
# or
wstunnel --config config.toml client
# or
wstunnel --config config.json client
```
