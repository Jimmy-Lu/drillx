#include <stdio.h>
#include <stdint.h>
#include "utils.h"

int number_multi_processors;
int number_blocks;
int number_threads;
int max_threads_per_mp;
int batch_size;

// Greatest common denominator
// Used in gpu_init() to calculate block_size
int gcd(int a, int b)
{
    return (a == 0) ? b : gcd(b % a, a);
}

// Initializes gpu parameters
extern "C" void gpu_init(uint32_t batchsize)
{
    cudaDeviceProp device_prop;
    int block_size;

    cudaError_t cudaerr = cudaGetDeviceProperties(&device_prop, 0);
    if (cudaerr != cudaSuccess)
    {
        printf("getting properties for device failed with error \"%s\".\n", cudaGetErrorString(cudaerr));
        exit(EXIT_FAILURE);
    }

    number_threads = min(device_prop.maxThreadsPerBlock, 256);
    number_multi_processors = device_prop.multiProcessorCount;
    max_threads_per_mp = device_prop.maxThreadsPerMultiProcessor;
    block_size = max_threads_per_mp / gcd(max_threads_per_mp, number_threads);
    number_blocks = block_size * number_multi_processors;
    batch_size = batchsize;
}

__device__ uint64_t saturating_add(uint64_t a, uint64_t b)
{
    uint64_t result = a + b;
    if (result < a)
    {
        return UINT64_MAX;
    }
    return result;
}

__device__ uint32_t difficulty(const uint8_t *hash)
{
    uint32_t count = 0;
    for (int i = 0; i < 32; i++)
    {
        uint32_t lz = __clz((int)hash[i]) - 24; // __clz counts leading zeros of a 32-bit int, adjust for 8-bit value

        count += lz;
        if (lz < 8)
        {
            break;
        }
    }
    return count;
}

__global__ void test_difficulty(const uint8_t *hash, uint32_t *result)
{
    *result = difficulty(hash);
}
