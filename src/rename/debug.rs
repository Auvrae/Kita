#[derive(Clone, Debug)]
pub struct DebugStats {
    pub frame_total_calc_time: Vec<u32>,
    pub gui_browser_calc_time: Vec<u32>,
    pub gui_modifier_calc_time: Vec<u32>,
    pub gui_selector_calc_time: Vec<u32>,
    pub thread_modifier_calc_time: Vec<u32>,
}
impl DebugStats {
    pub fn push(&mut self, value: u32, stat_type: DebugStatType) {
         match stat_type {
            DebugStatType::FrameTotal => {
                self.frame_total_calc_time.push(value);
                if self.frame_total_calc_time.len() == 129 {
                    self.frame_total_calc_time.remove(0);
                }
            },
            DebugStatType::GuiBrowser => {
                self.gui_browser_calc_time.push(value);
                if self.gui_browser_calc_time.len() == 129 {
                    self.gui_browser_calc_time.remove(0);
                }
            },
            DebugStatType::GuiModifier => {
                self.gui_modifier_calc_time.push(value);
                if self.gui_modifier_calc_time.len() == 129 {
                    self.gui_modifier_calc_time.remove(0);
                }
            },
            DebugStatType::GuiSelector => {
                self.gui_selector_calc_time.push(value);
                if self.gui_selector_calc_time.len() == 129 {
                    self.gui_selector_calc_time.remove(0);
                }
            },
            DebugStatType::ThreadModifier => {
                self.thread_modifier_calc_time.push(value);
                if self.thread_modifier_calc_time.len() == 129 {
                    self.thread_modifier_calc_time.remove(0);
                }
            }
        };
    }
}

pub enum DebugStatType {
    FrameTotal,
    GuiBrowser,
    GuiModifier,
    GuiSelector,
    ThreadModifier
}