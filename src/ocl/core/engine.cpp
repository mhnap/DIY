#include "engine.hpp"

#if defined(__APPLE__)
#include <OpenCL/opencl.h>
#else
#include <CL/opencl.h>
#endif

#include <fstream>
#include <sstream>
#include <stdexcept>
#include <utility>

namespace ocl {

Engine::Engine(std::string_view kernelName, std::vector<size_t> globalWorkSizes)
    : m_kernelName(kernelName), m_globalWorkSizes(std::move(globalWorkSizes)) {}

void Engine::setLocalWorkSizes(std::vector<size_t> localWorkSizes) {
  m_localWorkSizes = std::move(localWorkSizes);
}

void Engine::setData(void* input, void* output, size_t size, DataType type) {
  m_data = {input, output, size, type};
  addCompilerDefineOption("DATA_TYPE", dataTypeToString(type));
}

void Engine::addCompilerOption(std::string_view option) {
  if (!m_compilerOptions.empty()) {
    m_compilerOptions += " ";
  }
  m_compilerOptions += option;
}

void Engine::addCompilerDefineOption(std::string_view name, std::string_view definition) {
  std::string option = "-D ";
  option += name;
  option += '=';
  option += definition;
  addCompilerOption(option);
}

void Engine::run() {
  // Connect to a compute device
  cl_device_id deviceId;
  int gpu = 1;
  int err = clGetDeviceIDs(NULL, gpu ? CL_DEVICE_TYPE_GPU : CL_DEVICE_TYPE_CPU, 1, &deviceId, NULL);
  if (err != CL_SUCCESS) {
    throw std::runtime_error("OpenCL Error: Failed to create a device group! " + std::to_string(err));
  }

  // Create a compute context
  cl_context context = clCreateContext(0, 1, &deviceId, NULL, NULL, &err);
  if (!context) {
    throw std::runtime_error("OpenCL Error: Failed to create a compute context! " + std::to_string(err));
  }

  // Create a command queue
  cl_command_queue commands = clCreateCommandQueue(context, deviceId, 0, &err);
  if (!commands) {
    throw std::runtime_error("OpenCL Error: Failed to create a command queue! " + std::to_string(err));
  }

  // Load kernel source from file
  std::string kernelSource = loadKernelSource();
  const char* kernelSourcePtr = kernelSource.c_str();

  // Create compute program from the source buffer
  cl_program program = clCreateProgramWithSource(context, 1, (const char**)&kernelSourcePtr, NULL, &err);
  if (!program) {
    throw std::runtime_error("OpenCL Error: Failed to create compute program! " + std::to_string(err));
  }

  // Build the program executable
  err = clBuildProgram(program, 0, NULL, m_compilerOptions.c_str(), NULL, NULL);
  if (err != CL_SUCCESS) {
    size_t len;
    char buffer[2048];
    clGetProgramBuildInfo(program, deviceId, CL_PROGRAM_BUILD_LOG, sizeof(buffer), buffer, &len);
    throw std::runtime_error("OpenCL Error: Failed to build program executable! " + std::to_string(err) +
                             '\n' + std::string(buffer));
  }

  // Create compute kernel in the program we wish to run
  cl_kernel kernel = clCreateKernel(program, m_kernelName.data(), &err);
  if (!kernel || err != CL_SUCCESS) {
    throw std::runtime_error("OpenCL Error: Failed to create compute kernel! " + std::to_string(err));
  }

  cl_mem inputMem; // device memory used for the input array
  cl_mem outputMem; // device memory used for the output array

  if (m_data) {
    size_t dataSize = dataTypeToSize(m_data->type) * m_data->size;
    // Create the input and output arrays in device memory for our calculation
    inputMem = clCreateBuffer(context, CL_MEM_READ_ONLY, dataSize, NULL, NULL);
    outputMem = clCreateBuffer(context, CL_MEM_WRITE_ONLY, dataSize, NULL, NULL);
    if (!inputMem || !outputMem) {
      throw std::runtime_error("OpenCL Error: Failed to allocate device memory! " + std::to_string(err));
    }

    // Write our data set into the inputMem array in device memory
    err = clEnqueueWriteBuffer(commands, inputMem, CL_TRUE, 0, dataSize, m_data->input, 0, NULL, NULL);
    if (err != CL_SUCCESS) {
      throw std::runtime_error("OpenCL Error: Failed to write to source array! " + std::to_string(err));
    }

    // Set the arguments to our compute kernel
    err = 0;
    err = clSetKernelArg(kernel, 0, sizeof(cl_mem), &inputMem);
    err |= clSetKernelArg(kernel, 1, sizeof(cl_mem), &outputMem);
    if (err != CL_SUCCESS) {
      throw std::runtime_error("OpenCL Error: Failed to set kernel arguments! " + std::to_string(err));
    }
  }

  // Execute the kernel over the entire range of our 1d inputMem data set
  // using the maximum number of work group items for this device
  err = clEnqueueNDRangeKernel(commands, kernel, m_globalWorkSizes.size(), NULL, m_globalWorkSizes.data(),
                               m_localWorkSizes.empty() ? NULL : m_localWorkSizes.data(), 0, NULL, NULL);
  if (err) {
    throw std::runtime_error("OpenCL Error: Failed to execute kernel! " + std::to_string(err));
  }

  // Wait for the command commands to get serviced before reading back results
  clFinish(commands);

  if (m_data) {
    size_t dataSize = dataTypeToSize(m_data->type) * m_data->size;
    // Read back the results from the device to verify the outputMem
    err = clEnqueueReadBuffer(commands, outputMem, CL_TRUE, 0, dataSize, m_data->output, 0, NULL, NULL);
    if (err != CL_SUCCESS) {
      throw std::runtime_error("OpenCL Error: Failed to read output array! " + std::to_string(err));
    }

    // Shutdown and cleanup
    clReleaseMemObject(inputMem);
    clReleaseMemObject(outputMem);
  }

  clReleaseProgram(program);
  clReleaseKernel(kernel);
  clReleaseCommandQueue(commands);
  clReleaseContext(context);
}

std::string Engine::getKernelFilePath() const {
  std::string sourceFile = __FILE__;
  std::string kernelFilePath = sourceFile.substr(0, sourceFile.rfind('/'));
  kernelFilePath = kernelFilePath.substr(0, kernelFilePath.rfind('/'));
  kernelFilePath += "/kernels/" + m_kernelName + ".cl";
  return kernelFilePath;
}

std::string Engine::loadKernelSource() const {
  // Open file
  std::ifstream file(getKernelFilePath(), std::ios::in);
  if (!file.is_open()) {
    throw std::runtime_error("OpenCL Error: Failed to load kernel source!");
  }

  // Get source string
  std::ostringstream ss;
  ss << file.rdbuf();
  return ss.str();
}

} // namespace ocl