#include <cstdio>
#include <cufft.h>
#include <cuda_runtime.h>

#define CUDA_CHECK(call)                                                  \
    {                                                                     \
        cudaError_t err = call;                                           \
        if (err != cudaSuccess)                                           \
        {                                                                 \
            fprintf(stderr, "CUDA Error: %s\n", cudaGetErrorString(err)); \
            return -1;                                                    \
        }                                                                 \
    }

__global__ void normalize(cufftComplex *data, int N)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < N)
    {
        data[idx].x /= N; // normalize real part
        data[idx].y /= N; // normalize imaginary part
    }
}

extern "C" void *unified_malloc(size_t size)
{
    void *ptr;
    cudaError_t err = cudaMallocManaged(&ptr, size);
    if (err != cudaSuccess)
    {
        fprintf(stderr, "CUDA Error: %s\n", cudaGetErrorString(err));
        // lmao totally fine..don't worry.
        return NULL;
    }
    return ptr;
}

extern "C" size_t unified_free(void *ptr)
{
    CUDA_CHECK(cudaFree(ptr));
    return 0;
}

extern "C" size_t perform_cuda_unified(float *buffer, size_t buffer_size, cufftComplex *output)
{
    const int BATCH = 16; // arbitrarily chosen
    cufftHandle plan;
    if (cufftPlan1d(&plan, buffer_size, CUFFT_R2C, BATCH) != CUFFT_SUCCESS)
    {
        fprintf(stderr, "CUFFT Error: Plan creation failed\n");
        return EXIT_FAILURE;
    }

    if (cufftExecR2C(plan, buffer, output) != CUFFT_SUCCESS)
    {
        fprintf(stderr, "CUFFT Error: ExecR2C failed\n");
        return EXIT_FAILURE;
    }

    int blockSize = 256;
    int total_output = (buffer_size / 2 + 1) * BATCH;
    int numBlocks = (total_output + blockSize - 1) / blockSize;
    normalize<<<numBlocks, blockSize>>>(output, total_output);

    CUDA_CHECK(cudaDeviceSynchronize());

    cufftDestroy(plan);

    return 0;
}
