# A Libadwaita for Rust demo!

This demonstrates basic usage of GNOME Builder and Rust.

## Installation:

### Flatpak:

There's a flatpak in (Releases)[https://github.com/code-leech/SimpleApplication/releases/tag/release].

### Manual/Source

**Requirements:**
> Meson
> Cargo
> GTK & Libadwaita Libraries

First off, download the repo and head to the root folder.
If you have **root permission** then run:
```
meson setup builddir --prefix /usr --buildtype release
ninja -C builddir
sudo ninja -C builddir install
```

Else, considering that your home directory if in $PATH:
```
meson setup builddir --prefix ~/.local --buildtype release
ninja -C builddir
sudo ninja -C builddir install
```

**To uninstall run:** (in the root directory) `(sudo if required) ninja -C builddir uninstall`

Enjoy!
