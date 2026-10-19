use core::{num::NonZero, range::Range};

const FDT_MAGIC: [u8; 4] = 0xd00_dfeed_u32.to_be_bytes();
const FDT_TOTALSIZE_OFF: usize = 0x4;

const ZIMAGE_MAGIC: [u8; 4] = 0x016f_2818_u32.to_le_bytes();
const ZIMAGE_MAGIC_OFF: usize = 0x24;
const ZIMAGE_START_OFF: usize = 0x28;
const ZIMAGE_END_OFF: usize = 0x2c;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BootPlan {
    #[cfg(feature = "chainload")]
    Chainload {
        addr: NonZero<usize>
    },
    Kernel {
        kernel: NonZero<usize>,
        device_tree: NonZero<usize>,
        // initramfs: Option<core::range::Range<NonZero<usize>>>,
    },
}

impl BootPlan {
    pub unsafe fn scan() -> Option<Self> {
        // vanda's end address.
        unsafe extern "C" {
            static __payload_start: u8;
        }

        // unsure if this is needed, todo run miri
        let mut ptr =
            core::ptr::with_exposed_provenance((&raw const __payload_start).expose_provenance());

        let mut kernel: Option<NonZero<usize>> = None;
        let mut device_tree: Option<NonZero<usize>> = None;
        let initramfs: Option<NonZero<usize>> = None;

        while let Some(payload) = unsafe { Payload::try_from_ptr(ptr) } {
            let len = match payload {
                #[cfg(feature = "chainload")]
                Payload::Vanda { len: _ } => {
                    log::info!("found chainload image @ {:#x}", ptr.addr());
                    return Some(Self::Chainload {
                        // SAFETY: __payload_start's address is guaranteed to be nonzero because it starts at the end of our image.
                        addr: unsafe { NonZero::new(ptr as usize).unwrap_unchecked() },
                    });
                },
                #[cfg(feature = "kernel-zimage")]
                Payload::ZImage { len } => {
                    if kernel.is_none() {
                        log::info!("found zImage @ {:#x}", ptr.addr());
                        kernel = NonZero::new(ptr.addr());
                    } else {
                        log::warn!("skipped duplicate kernel @ {:#x}", ptr.addr());
                    }

                    len
                },
                #[cfg(feature = "kernel-image")]
                Payload::Image => {
                    if kernel.is_none() {
                        log::info!("found Image @ {:#x}", ptr.addr());
                        kernel = NonZero::new(ptr.addr());
                    } else {
                        log::warn!("skipped duplicate kernel @ {:#x}", ptr.addr());
                    }

                    // uncompressed ARM32 kernels have no header or metadata to work off of, so
                    // they must be the last in the payload chain.
                    break;
                }
                Payload::DeviceTree { len } => {
                    if device_tree.is_none() {
                        log::info!("found fdt @ {:#x}", ptr.addr());
                        device_tree = NonZero::new(ptr.addr());
                    } else {
                        log::warn!("skipped duplicate device tree @ {:#x}", ptr.addr());
                    }

                    len
                },
                // Payload::Initramfs { address_range } => {
                //     if kernel.is_none() {
                //         log::info!("found initramfs @ {:#x}", ptr.addr());
                //         kernel = NonZero::new(ptr.addr()..);
                //     } else {
                //         log::warn!("skipped duplicate initramfs @ {:#x}", ptr.addr());
                //     }

                //     0
                // },
            };
            
            unsafe {
                ptr = ptr.add(len);
            }
        }

        if let (Some(kernel), Some(device_tree)) = (kernel, device_tree) {
            Some(Self::Kernel {
                kernel,
                device_tree,
                // initramfs,
            })
        } else {
            None
        }
    }
}

pub enum Payload {
    /// vanda chainload image
    #[cfg(feature = "chainload")]
    Vanda { len: usize },
    
    /// Flattened devicetree blob
    DeviceTree { len: usize },
    
    /// uncompressed kernel image
    #[cfg(feature = "kernel-image")]
    Image,
    
    /// compressed kernel image
    #[cfg(feature = "kernel-zimage")]
    ZImage { len: usize },

    /// Flattened image tree
    #[cfg(feature = "kernel-fitimage")]
    FitImage {},
    
    // /// Initial RAM filesystem
    // Initramfs { address_range: Range<usize> },
}

unsafe fn read4(ptr: *const u8) -> [u8; 4] {
    unsafe { core::ptr::read_volatile(ptr as *const [u8; 4]) }
}

impl Payload {
    /// # Safety
    /// 
    /// `ptr..ptr+0x30` must be readable.
    unsafe fn try_from_ptr(ptr: *const u8) -> Option<Payload> {
        // devicetree
        if unsafe { read4(ptr) } == FDT_MAGIC {
            return Some(Self::DeviceTree {
                len: u32::from_be_bytes(unsafe { read4(ptr.add(FDT_TOTALSIZE_OFF)) }) as usize
            });
        }

        // zImage
        #[cfg(feature = "kernel-zimage")]
        if unsafe { read4(ptr.add(ZIMAGE_MAGIC_OFF)) } == ZIMAGE_MAGIC {
            let start = u32::from_le_bytes(unsafe {
                read4(ptr.add(ZIMAGE_START_OFF))
            });
            let end = u32::from_le_bytes(unsafe {
                read4(ptr.add(ZIMAGE_END_OFF))
            });

            return Some(Self::ZImage {
                len: end.checked_sub(start)? as usize,
            });
        }
        
        cfg_select! {
            // assume that any invalid payload is an uncompressed kernel
            feature = "kernel-image" => {
                Some(Self::Image);
            }

            // nothing
            not(feature = "kernel-zimage") => {
                None
            }
        }
    }
}
