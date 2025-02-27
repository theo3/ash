#![allow(dead_code)]
use crate::version::{DeviceV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct Maintenance1 {
    handle: vk::Device,
    fns: vk::KhrMaintenance1Fn,
}

impl Maintenance1 {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> Maintenance1 {
        let fns = vk::KhrMaintenance1Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Maintenance1 {
            handle: device.handle(),
            fns,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrMaintenance1Fn::name()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPoolKHR.html>"]
    unsafe fn trim_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolTrimFlagsKHR,
    ) {
        self.fns
            .trim_command_pool_khr(self.handle, command_pool, flags);
    }

    pub fn fp(&self) -> &vk::KhrMaintenance1Fn {
        &self.fns
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
