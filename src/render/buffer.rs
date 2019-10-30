use rendy::{
    hal,
    factory::Factory,
    memory::Dynamic,
};

use std::mem;

pub fn alloc_simple<B: hal::Backend, T>(factory: &Factory<B>, usage: hal::buffer::Usage, data: &[T]) -> Result<Escape<Buffer<B>>, failure::Error> {
    let mut buffer = factory
        .create_buffer(
            BufferInfo {
                size: (data.len() * mem::size_of::<T>()) as u64,
                usage: hal::buffer::Usage::VERTEX,
            },
            Dynamic,
        )?;

    unsafe {
        factory
            .upload_visible_buffer(
                &mut buffer,
                0,
                data,
            )?;
    }
    Ok(buffer)
}