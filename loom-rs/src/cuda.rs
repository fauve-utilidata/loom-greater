#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct cfftComplex {
    real: libc::c_float,
    imaginary: libc::c_float,
}

#[link(name = "loom")]
unsafe extern "C" {
    fn unified_malloc(size: libc::size_t) -> *mut libc::c_void;

    fn unified_free(ptr: *mut libc::c_void);

    fn perform_cuda_unified(
        buffer: *const libc::c_float,
        buffer_size: libc::size_t,
        output: *mut cfftComplex,
    );

}

pub fn run(input_data: &[f32]) -> Vec<cfftComplex> {
    unsafe {
        let batch = 16;
        let fft_size = input_data.len() / batch;

        let input_ptr = unified_malloc(input_data.len() * size_of::<f32>()) as *mut f32;
        let output_ptr = unified_malloc(((fft_size / 2) + 1) * batch * size_of::<cfftComplex>())
            as *mut cfftComplex;
        std::ptr::copy_nonoverlapping(
            input_data.as_ptr(),
            input_ptr,
            input_data.len() * size_of::<f32>(),
        );
        perform_cuda_unified(input_ptr, input_data.len() * size_of::<f32>(), output_ptr);
        Vec::from_raw_parts(
            output_ptr,
            ((fft_size / 2) + 1) * batch * size_of::<cfftComplex>(),
            ((fft_size / 2) + 1) * batch * size_of::<cfftComplex>(),
        )
    }
}
