# systemd Setup for wstunnel

This guide shows how to run wstunnel as a systemd service.

## Installation

1. Copy the systemd unit files to the systemd directory:
```bash
sudo cp wstunnel-client@.service /etc/systemd/system/
sudo cp wstunnel-server@.service /etc/systemd/system/
```

2. Create the wstunnel user and group:
```bash
sudo useradd -r -s /bin/false wstunnel
```

3. Create directories for configuration and logs:
```bash
sudo mkdir -p /etc/wstunnel
sudo mkdir -p /var/log/wstunnel
sudo chown wstunnel:wstunnel /var/log/wstunnel
```

4. Place your configuration files in `/etc/wstunnel/`:
```bash
sudo cp config-client-example.yaml /etc/wstunnel/my-tunnel.yaml
sudo chown root:wstunnel /etc/wstunnel/my-tunnel.yaml
sudo chmod 640 /etc/wstunnel/my-tunnel.yaml
```

## Usage

The service uses instance naming (the `@` symbol), where the instance name corresponds to the config file name (without the `.yaml` extension).

### Start a client
If you have `/etc/wstunnel/my-tunnel.yaml`:
```bash
sudo systemctl start wstunnel-client@my-tunnel
```

### Enable auto-start on boot
```bash
sudo systemctl enable wstunnel-client@my-tunnel
```

### Check status
```bash
sudo systemctl status wstunnel-client@my-tunnel
```

### View logs
```bash
sudo journalctl -u wstunnel-client@my-tunnel -f
```

### Multiple tunnels
You can run multiple instances with different configs:
```bash
# Start multiple clients
sudo systemctl start wstunnel-client@tunnel1
sudo systemctl start wstunnel-client@tunnel2

# Start a server
sudo systemctl start wstunnel-server@server-config
```

## Security Notes

The systemd units include security hardening options:
- Runs as unprivileged `wstunnel` user
- Restricted filesystem access
- Limited system calls
- Memory execution protection
- Network namespace restrictions

If you need to bind to privileged ports (< 1024), consider using:
- `AmbientCapabilities=CAP_NET_BIND_SERVICE` in the service file
- Or use port forwarding/proxy (recommended)
