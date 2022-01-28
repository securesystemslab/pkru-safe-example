/// Library level annotations for adding PKRU-Safe instrumentation
#![feature(plugin, custom_attribute)]
#![feature(macros_in_extern)]
#![plugin(mpk_protector)]
#![mpk_protector]

extern crate libc;

/// Safe wrapper for call to external C function write_memory.
pub fn write_vec_wrapper(vec: &mut Vec<u64>) {
    unsafe {
        let mem_addr = vec.as_mut_ptr();
        write_memory(mem_addr);
    }
}

#[link(name="memwrite", kind="static")]
extern "C" {
    /// External untrusted function write_memory.
    fn write_memory(mem_addr: *mut u64);
}
