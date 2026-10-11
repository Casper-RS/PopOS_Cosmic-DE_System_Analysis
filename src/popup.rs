// SPDX-License-Identifier: GPL-3.0-only

//! Popup shown when `popup_mode` is enabled: one section per component,
//! each with a header, a larger run chart and a color legend.

use cosmic::{
    Element, Theme,
    iced::{Alignment, Length, Size},
    widget::{Column, Row, container, text},
};

use crate::{
    applet::{Message, SystemMonitorApplet, base_background},
    color::Color,
    components::{
        gpu::GpuData,
        run::{SimpleHistoryChart, SuperimposedHistoryChart},
    },
    config::{ComponentConfig, CpuView, IoView, PercentView},
};

const POPUP_WIDTH: f32 = 360.0;
const CHART_SIZE: Size = Size {
    width: POPUP_WIDTH,
    height: 64.0,
};

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    #[allow(clippy::cast_precision_loss)]
    let mut size = bytes as f64;
    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{bytes} {}", UNITS[0])
    } else {
        format!("{size:.1} {}", UNITS[unit_index])
    }
}

#[allow(clippy::cast_precision_loss)]
fn percent(used: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        used as f64 / total as f64 * 100.0
    }
}

fn cpu_color(vis: &[CpuView]) -> Color {
    match vis.first() {
        Some(
            CpuView::Run { color, .. }
            | CpuView::BarGlobal { color, .. }
            | CpuView::BarCores { color, .. },
        ) => *color,
        None => Color::accent_blue,
    }
}

/// (back, front) colors
fn percent_colors(vis: &[PercentView], default: (Color, Color)) -> (Color, Color) {
    match vis.first() {
        Some(
            PercentView::Run {
                color_back,
                color_front,
                ..
            },
        ) => (*color_back, *color_front),
        Some(
            PercentView::Bar {
                color_left,
                color_right,
                ..
            },
        ) => (*color_left, *color_right),
        Some(
            PercentView::RunBack { color, .. }
            | PercentView::RunFront { color, .. }
            | PercentView::BarLeft { color, .. }
            | PercentView::BarRight { color, .. },
        ) => (*color, *color),
        None => default,
    }
}

/// (back, front) colors
fn io_colors(vis: &[IoView], default: (Color, Color)) -> (Color, Color) {
    match vis.first() {
        Some(IoView::Run {
            color_back,
            color_front,
            ..
        }) => (*color_back, *color_front),
        Some(IoView::RunBack { color, .. } | IoView::RunFront { color, .. }) => (*color, *color),
        None => default,
    }
}

fn legend<'a>(color: Color, label: String) -> Element<'a, Message> {
    let swatch = container(Column::new()).width(10).height(10).style(move |theme: &Theme| {
        container::Style {
            background: Some(cosmic::iced::Color::from(color.as_cosmic_color(theme)).into()),
            border: cosmic::iced::Border {
                radius: 2.0.into(),
                ..Default::default()
            },
            ..container::Style::default()
        }
    });
    Row::new()
        .push(swatch)
        .push(text::caption(label))
        .spacing(6)
        .align_y(Alignment::Center)
        .into()
}

fn section<'a>(
    title: String,
    value: String,
    chart: Element<'a, Message>,
    legends: Vec<Element<'a, Message>>,
) -> Element<'a, Message> {
    let header = Row::new()
        .push(container(text::heading(title)).width(Length::Fill))
        .push(text::heading(value))
        .align_y(Alignment::Center);
    let chart = container(chart)
        .width(CHART_SIZE.width)
        .height(CHART_SIZE.height)
        .style(base_background);
    Column::new()
        .push(header)
        .push(chart)
        .push(Column::with_children(legends).spacing(2))
        .spacing(6)
        .width(POPUP_WIDTH)
        .into()
}

impl SystemMonitorApplet {
    pub fn popup_view(&'_ self) -> Element<'_, Message> {
        let sections: Vec<Element<'_, Message>> = self
            .config
            .components
            .iter()
            .flat_map(|component| match component {
                ComponentConfig::Cpu(vis) => vec![self.cpu_section(cpu_color(vis))],
                ComponentConfig::Mem(vis) => vec![self.mem_section(percent_colors(
                    vis,
                    (Color::accent_green, Color::accent_purple),
                ))],
                ComponentConfig::Net(vis) => vec![self.net_section(io_colors(
                    vis,
                    (Color::accent_red, Color::accent_yellow),
                ))],
                ComponentConfig::Disk(vis) => vec![self.disk_section(io_colors(
                    vis,
                    (Color::accent_orange, Color::accent_pink),
                ))],
                ComponentConfig::Gpu(vis) => {
                    let colors =
                        percent_colors(vis, (Color::accent_warm_grey, Color::accent_indigo));
                    self.gpus
                        .data()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, data)| self.gpu_section(idx, data, colors))
                        .collect()
                }
            })
            .collect();

        Column::with_children(sections)
            .spacing(16)
            .padding(16)
            .into()
    }

    fn cpu_section(&'_ self, color: Color) -> Element<'_, Message> {
        let usage = self.sys.global_cpu_usage();
        section(
            "CPU".into(),
            format!("{usage:.1}%"),
            SimpleHistoryChart::new(&self.global_cpu, 100.0, color).into(),
            vec![legend(
                color,
                format!("Usage ({} threads)", self.sys.cpus().len()),
            )],
        )
    }

    fn mem_section(&'_ self, (back, front): (Color, Color)) -> Element<'_, Message> {
        let (used, total) = (self.sys.used_memory(), self.sys.total_memory());
        let (swap_used, swap_total) = (self.sys.used_swap(), self.sys.total_swap());
        let swap_label = if swap_total == 0 {
            "Swap: not available".to_string()
        } else {
            format!(
                "Swap: {} / {}",
                format_bytes(swap_used),
                format_bytes(swap_total)
            )
        };
        section(
            "Memory".into(),
            format!("{:.1}%", percent(used, total)),
            SuperimposedHistoryChart::new(
                &self.swap, swap_total, &front, &self.ram, total, &back,
            )
            .into(),
            vec![
                legend(
                    back,
                    format!("RAM: {} / {}", format_bytes(used), format_bytes(total)),
                ),
                legend(front, swap_label),
            ],
        )
    }

    /// Histories hold bytes per tick, convert the latest one to bytes per second
    fn rate(history: &crate::history::History, interval_ms: u64) -> u64 {
        let last = history.iter().last().copied().unwrap_or(0);
        last.saturating_mul(1000) / interval_ms.max(1)
    }

    fn net_section(&'_ self, (back, front): (Color, Color)) -> Element<'_, Message> {
        let interval = self.config.sampling.net.update_interval;
        let down = Self::rate(&self.download, interval);
        let up = Self::rate(&self.upload, interval);
        section(
            "Network".into(),
            format!("↓ {}/s  ↑ {}/s", format_bytes(down), format_bytes(up)),
            SuperimposedHistoryChart::new_linked(&self.upload, &front, &self.download, &back)
                .into(),
            vec![
                legend(back, format!("Download: {}/s", format_bytes(down))),
                legend(front, format!("Upload: {}/s", format_bytes(up))),
            ],
        )
    }

    fn disk_section(&'_ self, (back, front): (Color, Color)) -> Element<'_, Message> {
        let interval = self.config.sampling.disk.update_interval;
        let read = Self::rate(&self.disk_read, interval);
        let write = Self::rate(&self.disk_write, interval);
        section(
            "Disk".into(),
            format!("R {}/s  W {}/s", format_bytes(read), format_bytes(write)),
            SuperimposedHistoryChart::new_linked(&self.disk_write, &front, &self.disk_read, &back)
                .into(),
            vec![
                legend(back, format!("Read: {}/s", format_bytes(read))),
                legend(front, format!("Write: {}/s", format_bytes(write))),
            ],
        )
    }

    fn gpu_section(
        &'_ self,
        idx: usize,
        data: GpuData,
        (back, front): (Color, Color),
    ) -> Element<'_, Message> {
        let title = if self.gpus.num_gpus() > 1 {
            format!("GPU {idx}")
        } else {
            "GPU".to_string()
        };
        section(
            title,
            format!("{}%", data.usage),
            SuperimposedHistoryChart::new(
                &self.vram[idx],
                data.total_vram,
                &front,
                &self.gpu_usage[idx],
                100,
                &back,
            )
            .into(),
            vec![
                legend(back, format!("Usage: {}%", data.usage)),
                legend(
                    front,
                    format!(
                        "VRAM: {} / {} ({:.1}%)",
                        format_bytes(data.used_vram),
                        format_bytes(data.total_vram),
                        percent(data.used_vram, data.total_vram)
                    ),
                ),
            ],
        )
    }
}
