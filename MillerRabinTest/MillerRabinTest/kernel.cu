#include "cuda_runtime.h"
#include "device_launch_parameters.h"

#include <stdio.h>

/*
 Miller-Rabin test
 if p = 2 then return (p is a prime)
 if p is even then return (p is not a prime)
 find smallest d such that p = 2^r + d + 1 (d is an odd number)
 randomly choose a such that 2 <= a <= p - 2
 x := a^d mod p
 if x == 1 or x == p - 1 then return (p is probably a prime)
 repeat r - 1 times:
   x := x^2 mod p
   if x == p - 1 then return (p is probably a prime)
 return (p is not a prime)
 There is at most 1/4 numbers a such that the algorithms return false answer.
 The complexity of Miller-Rabin prime test is O(log3p)
 There is also a deterministic polynomial algorithm for the primality test with
 a time complexity O(n12).
*/

cudaError_t isPrimeWithCuda(const int *p);

__global__ void millerRabinTestKernel(const int* p, bool* isPrime)
{
    printf("Thread %d: Testing if %d is prime...\n", threadIdx.x, *p);
    int a = threadIdx.x + 2; // Randomly chosen a such that 2 <= a <= p - 2
    int d = *p - 1;
    int r = 0;
    while (d % 2 == 0) {
        d /= 2;
        r++;
    }
    printf("Thread %d: a = %d, d = %d, r = %d\n", threadIdx.x, a, d, r);
    int x = 1;
    for (int i = 0; i < d; i++) {
        x = (x * a) % *p;
    }
    if (x == 1 || x == *p - 1) {
        *isPrime = true;
        printf("Thread %d: %d is probably prime (x == 1 or x == p - 1)\n", threadIdx.x, *p);
        return;
    }
    for (int i = 0; i < r - 1; i++) {
        x = (x * x) % *p;
        if (x == *p - 1) {
            *isPrime = true;
            printf("Thread %d: %d is probably prime (x == p - 1)\n", threadIdx.x, *p);
            return;
        }
    }
    *isPrime = false;
    printf("Thread %d: %d is not prime\n", threadIdx.x, *p);
    return;
}

void isPrimeCpu(int p) {
    if (p < 2) {
        printf("CPU: Input number must be greater than 1!\n");
        return;
    }
    if (p == 2) {
        printf("CPU: Is %d prime? true\n", p);
        return;
    }
    if (p % 2 == 0) {
        printf("CPU: Is %d prime? false\n", p);
        return;
    }

    for (int i = 3; i * i <= p; i += 2) {
        if (p % i == 0) {
            printf("CPU: Is %d prime? false\n", p);
            return;
        }
    }
    printf("CPU: Is %d prime? true\n", p);
}

int main()
{
    int p;
    printf("Enter a number to test if it is prime: ");
    scanf("%d", &p);

    // Add vectors in parallel.
    cudaError_t cudaStatus = isPrimeWithCuda(&p);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "isPrimeWithCuda failed!");
        return 1;
    }

    // cudaDeviceReset must be called before exiting in order for profiling and
    // tracing tools such as Nsight and Visual Profiler to show complete traces.
    cudaStatus = cudaDeviceReset();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceReset failed!");
        return 1;
    }

    // Check if the number is prime using CPU
    isPrimeCpu(p);
    return 0;
}

// Helper function for using CUDA to add vectors in parallel.
cudaError_t isPrimeWithCuda(const int *p) {
    if (p == NULL) {
        fprintf(stderr, "Input number is NULL!");
        return cudaErrorInvalidValue;
    }
    if (*p < 2) {
        fprintf(stderr, "Input number must be greater than 1!");
        return cudaErrorInvalidValue;
    }
    if (*p == 2) {
        printf("Is %d prime? true\n", *p);
        return cudaSuccess;
    }
    if (*p % 2 == 0) {
        printf("Is %d prime? false\n", *p);
        return cudaSuccess;
    }

    int* dev_p = 0;
    bool* dev_isPrime = 0;
    cudaError_t cudaStatus;

    // Choose which GPU to run on, change this on a multi-GPU system.
    cudaStatus = cudaSetDevice(0);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaSetDevice failed!  Do you have a CUDA-capable GPU installed?");
        goto Error;
    }

    // Allocate GPU buffers for the input number and the output result.
    cudaStatus = cudaMalloc((void**)&dev_p, sizeof(int));
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed!");
        goto Error;
    }

    cudaStatus = cudaMalloc((void**)&dev_isPrime, sizeof(bool));
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMalloc failed!");
        goto Error;
    }

    // Copy input vectors from host memory to GPU buffers.
    cudaStatus = cudaMemcpy(dev_p, p, sizeof(int), cudaMemcpyHostToDevice);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed!");
        goto Error;
    }

    // Launch a kernel on the GPU with one thread for each a.
    millerRabinTestKernel<<<1, *p - 4>>>(dev_p, dev_isPrime);

    // Check for any errors launching the kernel
    cudaStatus = cudaGetLastError();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "millerRabinTestKernel launch failed: %s\n", cudaGetErrorString(cudaStatus));
        goto Error;
    }
    
    // cudaDeviceSynchronize waits for the kernel to finish, and returns
    // any errors encountered during the launch.
    cudaStatus = cudaDeviceSynchronize();
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaDeviceSynchronize returned error code %d after launching millerRabinTestKernel!\n", cudaStatus);
        goto Error;
    }

    // Copy output vector from GPU buffer to host memory.
    bool isPrime;
    cudaStatus = cudaMemcpy(&isPrime, dev_isPrime, sizeof(bool), cudaMemcpyDeviceToHost);
    if (cudaStatus != cudaSuccess) {
        fprintf(stderr, "cudaMemcpy failed!");
        goto Error;
    }
    printf("Is %d prime? %s\n", *p, isPrime ? "true" : "false");

Error:
    cudaFree(dev_p);
    cudaFree(dev_isPrime);

    return cudaStatus;
}
