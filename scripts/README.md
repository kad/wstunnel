# Package Installation Scripts

This directory contains installation scripts for DEB and RPM packages.

## Scripts

### preinstall.sh
Executed before the package is installed.
- Creates the `wstunnel` system user and group if they don't exist

### postinstall.sh
Executed after the package is installed.
- Reloads systemd to recognize new service units
- Sets proper permissions on `/etc/wstunnel` directory
- Displays installation success message and usage instructions

### preremove.sh
Executed before the package is removed.
- Stops and disables all running wstunnel client and server instances

## Package Contents

The DEB and RPM packages include:

### Binary
- `/usr/bin/wstunnel` - Main binary

### Systemd Units
- `/usr/lib/systemd/system/wstunnel-client@.service` - Client service template
- `/usr/lib/systemd/system/wstunnel-server@.service` - Server service template

### Configuration
- `/etc/wstunnel/` - Configuration directory (created, owned by root:wstunnel)

### Documentation and Examples
- `/usr/share/doc/wstunnel/README.md`
- `/usr/share/doc/wstunnel/LICENSE`
- `/usr/share/doc/wstunnel/systemd-setup.md`
- `/usr/share/doc/wstunnel/config-file.md`
- `/usr/share/doc/wstunnel/examples/` - Configuration examples
  - `config-client-example.yaml`
  - `config-server-example.yaml`
  - `config.example.yaml`
  - `config.example.toml`
  - `restrictions.yaml`
  - `example-env-vars.sh`

## Building Packages

Packages are built automatically by goreleaser during the release process:

```bash
goreleaser release --snapshot --clean
```

This will create both `.deb` and `.rpm` packages in the `dist/` directory.
