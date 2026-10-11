# Configuring the applet

You can configure the applet editing the files in `~/.config/cosmic/dev.DBrox.CosmicSystemMonitor/v2/`.

The configuration is divided in 3 files:

- [Components](./Components.md): controls what resource monitors are displayed and how they are viewed
- [Sampling](./Sampling.md): controls the speed of the sampling and how many samples are stored for each of the resources monitored
- [Layout](./Layout.md): controls the padding and spacing of the components

Three single-value files are also available:

- `tooltip_enabled` (`true`/`false`): show tooltips when hovering the in-panel charts
- `popup_mode` (`true`/`false`, default `true`): show a single icon in the panel and the charts in a popup when it is clicked
- `popup_show_disk` (`true`/`false`, default `true`): show the Disk section in the popup (also toggled by the switch in its header)
