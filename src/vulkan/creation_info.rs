use ash::vk::PresentModeKHR;

use crate::result::Context;

#[derive(Clone, Copy)]
pub enum PresentModeInitMode {
    Find(PresentModeKHR),
    FindOrFallback {
        wanted_mode: PresentModeKHR,
        fallback: PresentModeKHR,
    },
}

impl PresentModeInitMode {
    pub fn find_present_mode(
        &self,
        present_modes: Vec<PresentModeKHR>,
    ) -> crate::Result<PresentModeKHR> {
        match self {
            PresentModeInitMode::Find(wanted_mode) => present_modes
                .into_iter()
                .find(|&mode| mode == *wanted_mode)
                .context(format!("Present mode {:?} is not supported", wanted_mode)),
            PresentModeInitMode::FindOrFallback {
                wanted_mode,
                fallback,
            } => Ok(present_modes
                .into_iter()
                .find(|&mode| mode == *wanted_mode)
                .unwrap_or(*fallback)),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DataStructureCreationInfo {
    pub swapchain_present_mode: PresentModeInitMode,
    pub frame_overlap: usize,
}
