# PopOS Cosmic-DE System Analysis

A system resource monitor applet for the COSMIC desktop (Pop!_OS 24.04 and later). It shows CPU, memory, network, disk and GPU usage. By default it takes up a single icon in the panel, and clicking the icon opens a popup with live charts and current values.

> **This is a fork.** The original applet is
> [**System Monitor Cosmic Applet**](https://github.com/D-Brox/cosmic-ext-applet-system-monitor)
> by [**D-Brox**](https://github.com/D-Brox). Credit for the applet itself goes to them; this
> fork only adds the changes listed below. If you don't need those changes, use the original.

## Screenshots

<!--
  To add or replace a screenshot: edit this file on GitHub, drag the image into the
  editor below the matching heading, and GitHub inserts the link for you.
-->

### Popup mode

<img width="396" alt="Popup mode with collapsible sections" src="https://github.com/user-attachments/assets/1939287b-e6bb-4d6e-a56f-7a6f7a2c0788" />

### Drive overview

<img width="367" alt="Drive overview in the Disk section" src="https://github.com/user-attachments/assets/679ce83f-6e6d-48ac-bfd7-0dae043b4f5a" />

### New change

<!-- Paste a screenshot of the latest change here -->

### Panel charts (popup mode off)

![screenshot of the in-panel charts](./res/screenshot.png)

## What's different from the original

| Change | Description |
|---|---|
| **Popup mode** | The panel shows one system-monitor icon instead of a row of charts. Clicking it opens a popup with a larger chart for every configured component, the current value in each header, and a color legend with details (RAM/swap used, download/upload rate, VRAM, …). It is on by default and can be switched off to get the original in-panel charts back (see [Configuration](#configuration)). |
| **Collapsible popup** | Every section header has an arrow (down = collapsed, up = open). Sections start collapsed, showing just the header with the current value, so the popup fits smaller screens. Click a header to open it. Open sections are saved in `popup_expanded`. |
| **Drives in the popup** | The Disk section lists the drives in use with their model, size and used space (LUKS/LVM layers are followed to the real drive). Drives that aren't mounted sit behind an expandable **Other drives** row. |
| **Localization** | All popup text and the panel chart tooltips are translated (English, Dutch, Czech, Hungarian, Polish, Brazilian Portuguese, Swedish). The applet now loads your desktop language; the original only ever loaded English. |
| **Ignore specific GPUs** | GPUs whose PCI slot is listed in the `SYSMON_IGNORE_GPUS` environment variable are skipped. This is useful on desktops where an idle integrated GPU would otherwise show up next to the discrete one. |
| **Full-width charts** | Charts fill their area instead of being cut off at 100 px, which showed in the larger popup charts. |
| **Flatpak manifest** | `res/dev.DBrox.CosmicSystemMonitor.json` now builds the local checkout and uses the `org.freedesktop.Platform` 25.08 runtime. |

Everything else (components, colors, sampling, layout) behaves exactly like the original
and is configured the same way.

The app ID is still `dev.DBrox.CosmicSystemMonitor`, so this fork replaces the original
applet if it is installed and reuses its configuration.

## Installing

### From the release (Flatpak)

1. Download the `.flatpak` file from the [latest release](https://github.com/Casper-RS/PopOS_Cosmic-DE_System_Analysis/releases/latest).
2. If the original applet is installed from the COSMIC Flatpak repository, remove it first
   (both use the same app ID):

   ```sh
   flatpak uninstall dev.DBrox.CosmicSystemMonitor
   ```

3. Install the bundle. The required runtime is pulled from Flathub automatically:

   ```sh
   flatpak install --user ./cosmic-system-monitor-*.flatpak
   ```

4. Add the applet to a panel: **Settings → Desktop → Panel (or Dock) → Configure panel applets → Add applet → System Monitor**.

If you are upgrading an applet that is already in the panel, restart the panel to load the
new version (log out and back in, or run `pkill -x cosmic-panel`; COSMIC restarts it).

### Building from source

Clone the repository:

```sh
git clone https://github.com/Casper-RS/PopOS_Cosmic-DE_System_Analysis
cd PopOS_Cosmic-DE_System_Analysis
```

**Native build.** Install the build dependencies (`rustc`/`cargo` 1.91 or newer, `just`, `libxkbcommon-dev`), then:

```sh
just build-release
sudo just install
# or
just install-local
```

For other packaging methods:

- `deb`: run `just build-deb` and `sudo just install-deb`
- `rpm`: run `just build-rpm` and `sudo just install-rpm`

For vendoring, use `just vendor` and `just vendor-build`.

**Flatpak build.** Install `flatpak-builder` (or the `org.flatpak.Builder` Flatpak) together with
the `org.freedesktop.Sdk//25.08` and `org.freedesktop.Sdk.Extension.rust-stable//25.08` runtimes, then:

```sh
flatpak-builder --user --install --force-clean builddir res/dev.DBrox.CosmicSystemMonitor.json
```

## Configuration

Configuration lives in `~/.config/cosmic/dev.DBrox.CosmicSystemMonitor/v2/`. The applet picks up
changes right away. The original options are documented in [docs](./docs/README.md).

### Popup mode

```sh
# one icon + popup (default)
echo true  > ~/.config/cosmic/dev.DBrox.CosmicSystemMonitor/v2/popup_mode
# original behaviour: charts directly in the panel
echo false > ~/.config/cosmic/dev.DBrox.CosmicSystemMonitor/v2/popup_mode
```

The popup shows the components from the `components` file in the same order and uses
their colors.

### Ignoring a GPU

Find the PCI slot of the GPU you want to hide:

```sh
grep -H PCI_SLOT_NAME /sys/class/drm/card*/device/uevent
```

Then set it, comma-separating multiple slots. For the Flatpak:

```sh
flatpak override --user --env=SYSMON_IGNORE_GPUS=0000:12:00.0 dev.DBrox.CosmicSystemMonitor
```

For a native install, export `SYSMON_IGNORE_GPUS` in the environment that starts your session.
Restart the panel afterwards.

## Roadmap

Theming:
- [x] Layout
- [x] Custom colors
- [ ] Transparency

Resource monitoring:
- [x] CPU usage (global and per core)
- [x] Memory usage (RAM and Swap)
- [x] Network I/O
- [x] Disk I/O
- [x] GPU (usage and VRAM)
- [ ] Thermal sensors

[Component](./docs/Components.md) views
- [x] Run chart views (percentage and I/O)
- [x] Bar chart views (percentage and CPU cores)
- [x] Popup with charts and current values *(this fork)*
- [ ] Text views

## Credits

- [**D-Brox**](https://github.com/D-Brox): original author of the
  [System Monitor Cosmic Applet](https://github.com/D-Brox/cosmic-ext-applet-system-monitor), which this fork is based on
- All [contributors to the original project](https://github.com/D-Brox/cosmic-ext-applet-system-monitor/graphs/contributors)
- [Casper-RS](https://github.com/Casper-RS): popup mode and GPU ignore list in this fork

The original project thanks:

- [paradoxxxzero](https://github.com/paradoxxxzero) for their [GNOME Shell system monitor extension](https://github.com/paradoxxxzero/gnome-shell-system-monitor-applet), the inspiration for this applet
- [edfloreshz](https://github.com/edfloreshz) for the [template for COSMIC applets](https://github.com/edfloreshz/cosmic-applet-template), which taught me the logic behind an applet
- [aschiavon91](https://github.com/aschiavon91) for their initial work at a [system status applet](https://github.com/aschiavon91/cosmic-applet-sys-status/), which was used as a reference implementation

## License

[GPL-3.0](./LICENSE), the same as the original project. This fork is a modified version of
the original. The changes are described above, and the full history is in the git log.
