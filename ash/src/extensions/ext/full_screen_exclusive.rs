#![allow(dead_code)]
use crate::prelude::*;
use crate::version::{DeviceV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct FullScreenExclusive {
    handle: vk::Device,
    full_screen_exclusive_fn: vk::ExtFullScreenExclusiveFn,
}

impl FullScreenExclusive {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> FullScreenExclusive {
        let full_screen_exclusive_fn = vk::ExtFullScreenExclusiveFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        FullScreenExclusive {
            handle: device.handle(),
            full_screen_exclusive_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::ExtFullScreenExclusiveFn::name()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html>"]
    pub unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        self.full_screen_exclusive_fn
            .acquire_full_screen_exclusive_mode_ext(self.handle, swapchain)
            .result()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>"]
    pub unsafe fn get_physical_device_surface_present_modes2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        let mut count = mem::zeroed();
        self.full_screen_exclusive_fn
            .get_physical_device_surface_present_modes2_ext(
                physical_device,
                surface_info,
                &mut count,
                ptr::null_mut(),
            )
            .result()?;
        let mut present_modes = Vec::<vk::PresentModeKHR>::with_capacity(count as usize);
        let err_code = self
            .full_screen_exclusive_fn
            .get_physical_device_surface_present_modes2_ext(
                physical_device,
                surface_info,
                &mut count,
                present_modes.as_mut_ptr(),
            );
        present_modes.set_len(count as usize);
        err_code.result_with_success(present_modes)
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html>"]
    pub unsafe fn release_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        self.full_screen_exclusive_fn
            .release_full_screen_exclusive_mode_ext(self.handle, swapchain)
            .result()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>"]
    pub unsafe fn get_device_group_surface_present_modes2(
        &self,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut present_modes = mem::zeroed();
        self.full_screen_exclusive_fn
            .get_device_group_surface_present_modes2_ext(
                self.handle,
                surface_info,
                &mut present_modes,
            )
            .result_with_success(present_modes)
    }

    pub fn fp(&self) -> &vk::ExtFullScreenExclusiveFn {
        &self.full_screen_exclusive_fn
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
