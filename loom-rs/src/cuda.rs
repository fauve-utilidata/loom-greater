#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct cfftComplex {
    real: libc::c_float,
    imaginary: libc::c_float,
}

#[link(name = "loom")]
unsafe extern "C" {
    fn perform_cuda_fft(
        buffer: *const libc::c_float,
        buffer_size: libc::size_t,
        output: *mut cfftComplex,
    ) -> libc::size_t;

    fn unified_malloc(size: libc::size_t) -> *mut libc::c_void;

    fn unified_free(ptr: *mut libc::c_void);

    fn perform_cuda_unified(
        buffer: *const libc::c_float,
        buffer_size: libc::size_t,
        output: *mut cfftComplex,
    );

}
