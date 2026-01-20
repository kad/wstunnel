# Configuration File Support

wstunnel now supports YAML configuration files as an alternative to command-line arguments. This makes it easier to manage complex configurations and reuse them across deployments.

## Usage

Use the `--config` flag to specify a configuration file:

```bash
# Using a client config
wstunnel --config config-client.yaml client

# Using a server config
wstunnel --config config-server.yaml server
```

**Note:** CLI arguments take precedence over config file values. This allows you to override specific settings without modifying the config file.

## Configuration File Format

Configuration files are written in YAML and can contain either a `client` or `server` section (or both).

### Client Configuration Example

```yaml
client:
  # Local to remote tunnels (required)
  local_to_remote:
    - "tcp://8080:localhost:80"
    - "socks5://127.0.0.1:1080"
    - "udp://5353:1.1.1.1:53"
  
  # Server address (required)
  remote_addr: "wss://tunnel.example.com:443"
  
  # Connection settings
  connection_min_idle: 5
  connection_retry_max_backoff: "5m"
  
  # TLS settings
  tls_verify_certificate: true
  tls_certificate: "/path/to/client-cert.pem"
  tls_private_key: "/path/to/client-key.pem"
  
  # HTTP settings
  http_upgrade_path_prefix: "v1"
  http_headers:
    - "X-Custom-Header: value"
  
  # WebSocket settings
  websocket_ping_frequency: "30s"
  
  # DNS settings
  dns_resolver:
    - "dns://1.1.1.1"
    - "dns+https://1.1.1.1?sni=cloudflare-dns.com"
```

### Server Configuration Example

```yaml
server:
  # Bind address (required)
  remote_addr: "wss://0.0.0.0:8080"
  
  # TLS settings
  tls_certificate: "/path/to/server-cert.pem"
  tls_private_key: "/path/to/server-key.pem"
  
  # Access restrictions
  restrict_config: "/path/to/restrictions.yaml"
  
  # WebSocket settings
  websocket_ping_frequency: "30s"
  
  # Timeout settings
  remote_to_local_server_idle_timeout: "3m"
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

### Simple Client

```yaml
client:
  local_to_remote:
    - "tcp://3000:localhost:3000"
  remote_addr: "ws://tunnel.example.com:8080"
```

### Client with TLS and Custom Headers

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

1. **Version Control**: Store your config files in version control (excluding sensitive credentials)
2. **Environment-Specific Configs**: Create separate config files for development, staging, and production
3. **Security**: Use file permissions to protect config files containing credentials (e.g., `chmod 600 config.yaml`)
4. **Testing**: Test your configuration with `--log-lvl DEBUG` to see detailed connection information
5. **Validation**: The config file is validated on load - you'll get clear error messages if something is wrong

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

You can convert it to a config file:

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

And run it simply as:

```bash
wstunnel --config my-tunnel.yaml client
```
