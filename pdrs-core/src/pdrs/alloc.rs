use std::{alloc::GlobalAlloc, ptr::{null_mut}, ffi::c_void};
use crate::api::{
    PlaydateAPI,
    system::PlaydateSys
};

static mut PD_API: Option<&PlaydateAPI> = None;

pub unsafe fn set_pd_api(pd_api: *const PlaydateAPI){
    PD_API = pd_api.as_ref();
}

pub struct PDAllocator;

unsafe impl GlobalAlloc for PDAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        if PD_API.is_none() {
            return null_mut();
        }
        let pd_api = PD_API.unwrap();

        const PTR_SIZE: usize = std::mem::size_of::<*mut u8>();

        let size = layout.size();
        let align = layout.align();

        // max extra size to align + store memory
        let hdr_size = if align > 1 { PTR_SIZE + align - 1} else { 0 };

        let mem: *mut u8 = pd_api.system.as_ref()
            .map(|sys: &PlaydateSys|{
                std::mem::transmute::<*mut c_void, *mut u8>(
                    (sys.realloc)(
                        null_mut(),
                        size + hdr_size
                    )
                )
            })
            .unwrap_or(null_mut());

        if mem.is_null() || align == 1 {
            return mem;
        }

        let align_offset = mem.offset(PTR_SIZE as isize).align_offset(align);
        if align_offset == usize::MAX {
            return null_mut();
        }

        let aligned_mem = mem.offset((align_offset + PTR_SIZE) as isize);

        // store the pointer to the memory at the address before
        *(std::mem::transmute::<*mut u8, *mut *mut u8>(aligned_mem).offset(-1)) = mem;

        // (pd_api.system.as_ref().unwrap().log_to_console)(
        //     std::mem::transmute(b"PTR ALLOCED: %p\nPTR ALLOCED (ALIGNED): %p\0"),
        //     std::mem::transmute::<*mut u8, *mut c_void>(mem),
        //     std::mem::transmute::<*mut u8, *mut c_void>(aligned_mem)
        // );

        aligned_mem
    }

    unsafe fn dealloc(&self, aligned_mem: *mut u8, layout: std::alloc::Layout) {
        if PD_API.is_none() {
            return
        }

        let pd_api = PD_API.unwrap();

        // (pd_api.system.as_ref().unwrap().log_to_console)(
        //     std::mem::transmute(b"PTR TO DEALLOC: %p\0"),
        //     std::mem::transmute::<*mut u8, *mut c_void>(aligned_mem)
        // );

        let align = layout.align();

        let mem: *mut u8 = if align == 1 {
            aligned_mem
        } else {
            // go back one pointer size to get the location of the
            // allocated memory
            *(std::mem::transmute::<*mut u8, *mut *mut u8>(aligned_mem).offset(-1))
        };

        pd_api.system.as_ref()
            .map(|sys|{
                (sys.realloc)(std::mem::transmute(mem), 0)
            });
    }
}

#[global_allocator]
pub static PD_ALLOCATOR: PDAllocator = PDAllocator;