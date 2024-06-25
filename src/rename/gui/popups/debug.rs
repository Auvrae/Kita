use egui_plot;
use super::super::super::app::WindowMain;


pub fn window(gui: &mut WindowMain, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.vertical( |ui| {
        ui.group(|ui| {
            let mut frame_time: u32 = 0;
            let mut highest_time: u32 = 0;
            let mut lowest_time: u32 = 150000;
            for time in gui.statistics.frame_total_calc_time.iter().enumerate() {
                if time.1 > &highest_time {
                    highest_time = *time.1;
                };
                if time.1 < &lowest_time {
                    lowest_time = *time.1;
                }
                frame_time += time.1;
            };
            if frame_time >= 1 {
                frame_time = frame_time / gui.statistics.frame_total_calc_time.len() as u32;
            }
            ui.label(format!("Frametime in ms. Avg: {}, low: {}, high: {}", frame_time, lowest_time, highest_time));
            let mut bars: Vec<egui_plot::Bar> = vec![];
            for (index, time) in gui.statistics.frame_total_calc_time.iter().enumerate() {
                bars.push(egui_plot::Bar::new((150 * index) as f64, time.to_owned() as f64)
                    .width(1.0)
                    .fill(egui::Color32::from_hex("#6A1B9A18").unwrap())
                    .base_offset(4.0)
                )
            }
            let chart = egui_plot::BarChart::new(bars)
            .vertical()
            .width(200.0);
    
            egui_plot::Plot::new(egui::Id::from("bulk_frame_time"))
            .show_grid(false)
            .allow_boxed_zoom(false)
            .center_x_axis(false)
            .center_y_axis(false)
            .clamp_grid(true)
            .allow_scroll(false)
            .show_axes(egui::Vec2b::new(false, true))
            .y_axis_formatter(|mut grid, _, _| {
                grid.step_size = 2.0;
                String::from(format!("{}", grid.value))
            })
            .coordinates_formatter(
                egui_plot::Corner::LeftBottom, 
                egui_plot::CoordinatesFormatter::new(|point, _| {
                    String::from(format!("{}", point.y.floor()))
                }))
            .height(60.0)
            .show(ui, |ui| {
                ui.bar_chart(chart);
            })
        });

        ui.group(|ui| {
            let mut frame_time: u32 = 0;
            let mut highest_time: u32 = 0;
            let mut lowest_time: u32 = 150000;
            for time in gui.statistics.gui_browser_calc_time.iter().enumerate() {
                if time.1 > &highest_time {
                    highest_time = *time.1;
                };
                if time.1 < &lowest_time {
                    lowest_time = *time.1;
                }
                frame_time += time.1;
            };
            if frame_time >= 1 {
                frame_time = frame_time / gui.statistics.gui_browser_calc_time.len() as u32;
            }
            ui.label(format!("Gui Browser render time in μs. Avg: {}, low: {}, high: {}", frame_time, lowest_time, highest_time));
            let mut bars: Vec<egui_plot::Bar> = vec![];
            for (index, time) in gui.statistics.gui_browser_calc_time.iter().enumerate() {
                bars.push(egui_plot::Bar::new((150 * index) as f64, time.to_owned() as f64)
                    .width(1.0)
                    .fill(egui::Color32::from_hex("#28359318").unwrap())
                    .base_offset(4.0)
                )
            }
            let chart = egui_plot::BarChart::new(bars)
            .vertical()
            .width(200.0);
    
            egui_plot::Plot::new(egui::Id::from("plot_gui_render"))
            .show_grid(false)
            .allow_boxed_zoom(false)
            .center_x_axis(false)
            .center_y_axis(false)
            .clamp_grid(true)
            .allow_scroll(false)
            .show_axes(egui::Vec2b::new(false, true))
            .y_axis_formatter(|mut grid, _, _| {
                grid.step_size = 2.0;
                String::from(format!("{}", grid.value))
            })
            .coordinates_formatter(
                egui_plot::Corner::LeftBottom, 
                egui_plot::CoordinatesFormatter::new(|point, _| {
                    String::from(format!("{}", point.y.floor()))
                }))
            .height(60.0)
            .show(ui, |ui| {
                ui.bar_chart(chart);
            })
        });

        ui.group(|ui| {
            let mut frame_time: u32 = 0;
            let mut highest_time: u32 = 0;
            let mut lowest_time: u32 = 150000;
            for time in gui.statistics.gui_selector_calc_time.iter().enumerate() {
                if time.1 > &highest_time {
                    highest_time = *time.1;
                };
                if time.1 < &lowest_time {
                    lowest_time = *time.1;
                }
                frame_time += time.1;
            };
            if frame_time >= 1 {
                frame_time = frame_time / gui.statistics.gui_selector_calc_time.len() as u32;
            }
            ui.label(format!("Gui Selector render time in μs. Avg: {}, low: {}, high: {}", frame_time, lowest_time, highest_time));
            let mut bars: Vec<egui_plot::Bar> = vec![];
            for (index, time) in gui.statistics.gui_selector_calc_time.iter().enumerate() {
                bars.push(egui_plot::Bar::new((150 * index) as f64, time.to_owned() as f64)
                    .width(1.0)
                    .fill(egui::Color32::from_hex("#00695C18").unwrap())
                    .base_offset(4.0)
                )
            }
            let chart = egui_plot::BarChart::new(bars)
            .vertical()
            .width(200.0);
    
            egui_plot::Plot::new(egui::Id::from("plot_gui_selector"))
            .show_grid(false)
            .allow_boxed_zoom(false)
            .center_x_axis(false)
            .center_y_axis(false)
            .clamp_grid(true)
            .allow_scroll(false)
            .show_axes(egui::Vec2b::new(false, true))
            .y_axis_formatter(|mut grid, _, _| {
                grid.step_size = 2.0;
                String::from(format!("{}", grid.value))
            })
            .coordinates_formatter(
                egui_plot::Corner::LeftBottom, 
                egui_plot::CoordinatesFormatter::new(|point, _| {
                    String::from(format!("{}", point.y.floor()))
                }))
            .height(60.0)
            .show(ui, |ui| {
                ui.bar_chart(chart);
            })
        });

        ui.group(|ui| {
            let mut frame_time: u32 = 0;
            let mut highest_time: u32 = 0;
            let mut lowest_time: u32 = 150000;
            for time in gui.statistics.gui_modifier_calc_time.iter().enumerate() {
                if time.1 > &highest_time {
                    highest_time = *time.1;
                };
                if time.1 < &lowest_time {
                    lowest_time = *time.1;
                }
                frame_time += time.1;
            };
            if frame_time >= 1 {
                frame_time = frame_time / gui.statistics.gui_modifier_calc_time.len() as u32;
            }
            ui.label(format!("Gui Modifiers render time in μs. Avg: {}, low: {}, high: {}", frame_time, lowest_time, highest_time));
            let mut bars: Vec<egui_plot::Bar> = vec![];
            for (index, time) in gui.statistics.gui_modifier_calc_time.iter().enumerate() {
                bars.push(egui_plot::Bar::new((150 * index) as f64, time.to_owned() as f64)
                    .width(1.0)
                    .fill(egui::Color32::from_hex("#FF8F0018").unwrap())
                    .base_offset(4.0)
                )
            }
            let chart = egui_plot::BarChart::new(bars)
            .vertical()
            .width(200.0);
    
            egui_plot::Plot::new(egui::Id::from("plot_gui_modifier"))
            .show_grid(false)
            .allow_boxed_zoom(false)
            .center_x_axis(false)
            .center_y_axis(false)
            .clamp_grid(true)
            .allow_scroll(false)
            .show_axes(egui::Vec2b::new(false, true))
            .y_axis_formatter(|mut grid, _, _| {
                grid.step_size = 2.0;
                String::from(format!("{}", grid.value))
            })
            .coordinates_formatter(
                egui_plot::Corner::LeftBottom, 
                egui_plot::CoordinatesFormatter::new(|point, _| {
                    String::from(format!("{}", point.y.floor()))
                }))
            .height(60.0)
            .show(ui, |ui| {
                ui.bar_chart(chart);
            })
        });

        ui.group(|ui| {
            let mut frame_time: u32 = 0;
            let mut highest_time: u32 = 0;
            let mut lowest_time: u32 = 150000;
            for time in gui.statistics.thread_modifier_calc_time.iter().enumerate() {
                if time.1 > &highest_time {
                    highest_time = *time.1;
                };
                if time.1 < &lowest_time {
                    lowest_time = *time.1;
                }
                frame_time += time.1;
            };
            if frame_time >= 1 {
                frame_time = frame_time / gui.statistics.thread_modifier_calc_time.len() as u32;
            }
            ui.label(format!("Modifier thread calculation time in ms. Avg: {}, low: {}, high: {}", frame_time, lowest_time, highest_time));
            let mut bars: Vec<egui_plot::Bar> = vec![];
            for (index, time) in gui.statistics.thread_modifier_calc_time.iter().enumerate() {
                bars.push(egui_plot::Bar::new((150 * index) as f64, time.to_owned() as f64)
                    .width(1.0)
                    .fill(egui::Color32::from_hex("#D8431518").unwrap())
                    .base_offset(4.0)
                )
            }
            let chart = egui_plot::BarChart::new(bars)
            .vertical()
            .width(200.0);
    
            egui_plot::Plot::new(egui::Id::from("plot_thread_modifier"))
            .show_grid(false)
            .allow_boxed_zoom(false)
            .center_x_axis(false)
            .center_y_axis(false)
            .clamp_grid(true)
            .allow_scroll(false)
            .show_axes(egui::Vec2b::new(false, true))
            .y_axis_formatter(|mut grid, _, _| {
                grid.step_size = 2.0;
                String::from(format!("{}", grid.value))
            })
            .coordinates_formatter(
                egui_plot::Corner::LeftBottom, 
                egui_plot::CoordinatesFormatter::new(|point, _| {
                    String::from(format!("{}", point.y.floor()))
                }))
            .height(60.0)
            .show(ui, |ui| {
                ui.bar_chart(chart);
            })
        });
    });
}