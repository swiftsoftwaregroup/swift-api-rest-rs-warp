# Setup Podman on macOS

Podman is a utility that can be used to create and maintain containers. This post will teach you how to set up Podman on macOS and perform some basic commands.

<!-- more -->

## Preparation

### Homebrew

Install [Homebrew](https://brew.sh/):

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### bash 5

Install via Homebrew:

```bash
brew install bash
```

## Podman

Install via Homebrew:

```bash
brew install podman
```

## Podman Machine

Create and start your first Podman machine (2 CPUs, 100GB disk, 4GB memory). Podman machines are backed by [QEMU](https://www.qemu.org/). This will become the default Podman machine:


```bash
podman machine init --cpus 2 --disk-size 100 --memory 4096
```

List all machines:

```bash
podman machine ls
```

```
NAME                     VM TYPE     CREATED             LAST UP             CPUS        MEMORY      DISK SIZE
podman-machine-default*  qemu        About a minute ago  About a minute ago  2           4.295GB     107.4GB
```

Start the default machine:

```bash
podman machine start
```

Verify the installation:

```bash
podman info
```

The machine will be configured in rootless mode. If your containers require root permissions (e.g. ports < 1024), or if you run into compatibility issues with non-podman clients, you can switch using the following command: 

```bash
podman machine set --rootful
```

## Test

Run the `Hello World` container:

```bash
podman run hello-world
```

## Podman Desktop

To start a podman machine automatically at login, also install the Podman Desktop:

```bash
brew install --cask podman-desktop
```
