#include "engine.hpp"
#include "error.hpp"

#ifdef __APPLE__
#include <OpenCL/opencl.h>
#else
#include <CL/opencl.h>
#endif

#include <fstream>
#include <memory>
#include <sstream>
#include <utility>

namespace ocl {

Engine::Engine(std::string_view kernelName, std::vector<size_t> globalWorkSizes)
    : m_kernelName(kernelName), m_globalWorkSizes(std::move(globalWorkSizes)) {
  addCompilerOptionDefaultIncludeDirectories();
}

void Engine::setLocalWorkSizes(std::vector<size_t> localWorkSizes) {
  m_localWorkSizes = std::move(localWorkSizes);
}

void Engine::setData(const void* input, void* output, size_t size, DataType type) {
  m_data = {input, output, size, type};
  addCompilerOptionDefine("DATA_TYPE", dataTypeToString(type));
}

void Engine::addCompilerOption(std::string_view option) {
  if (!m_compilerOptions.empty()) {
    m_compilerOptions += " ";
  }
  m_compilerOptions += option;
}

void Engine::addCompilerOptionDefine(std::string_view name) {
  std::string option = "-D ";
  option += name;
  addCompilerOption(option);
}

void Engine::addCompilerOptionDefine(std::string_view name, std::string_view definition) {
  std::string option = "-D ";
  option += name;
  option += '=';
  option += definition;
  addCompilerOption(option);
}

void Engine::addCompilerOptionIncludeDirectory(std::string_view dir) {
  std::string option = "-I ";
  option += dir;
  addCompilerOption(option);
}

void Engine::enableProfiling() { m_isProfilingEnabled = true; }

std::chrono::nanoseconds Engine::getExecutionTime() const {
  if (!m_isProfilingEnabled) {
    throw EngineError("Cannot get execution time as profiling was not enabled");
  }
  return m_executionTime;
}

void Engine::run() {
  // Get platform id
  auto platformId = std::make_unique<cl_platform_id>();
  int err = clGetPlatformIDs(1, platformId.get(), nullptr);
  if (err != CL_SUCCESS) {
    throw OpenCLError("Failed to get platform id", err);
  }

  // Connect to a compute device
  cl_device_id deviceId = nullptr;
  const bool gpu = true;
  err = clGetDeviceIDs(*platformId, gpu ? CL_DEVICE_TYPE_GPU : CL_DEVICE_TYPE_CPU, 1, &deviceId, nullptr);
  if (deviceId == nullptr || err != CL_SUCCESS) {
    throw OpenCLError("Failed to create a device group", err);
  }

  // Create a compute context
  cl_context context = clCreateContext(nullptr, 1, &deviceId, nullptr, nullptr, &err);
  if (context == nullptr || err != CL_SUCCESS) {
    throw OpenCLError("Failed to create a compute context", err);
  }

  // Create a command queue
  cl_command_queue_properties command_queue_properties = 0;
  if (m_isProfilingEnabled) {
    command_queue_properties |= CL_QUEUE_PROFILING_ENABLE;
  }
  cl_command_queue commands = clCreateCommandQueue(context, deviceId, command_queue_properties, &err);
  if (commands == nullptr || err != CL_SUCCESS) {
    throw OpenCLError("Failed to create a command queue", err);
  }

  // Load kernel source from file
  const std::string kernelSource = loadKernelSource();
  const char* kernelSourcePtr = kernelSource.c_str();

  // Create compute program from the source buffer
  cl_program program = clCreateProgramWithSource(context, 1, (const char**)&kernelSourcePtr, nullptr, &err);
  if (program == nullptr || err != CL_SUCCESS) {
    throw OpenCLError("Failed to create compute program", err);
  }

  // Build the program executable
  err = clBuildProgram(program, 0, nullptr, m_compilerOptions.c_str(), nullptr, nullptr);
  if (err != CL_SUCCESS) {
    // Determine the size of the log
    size_t log_size = 0;
    clGetProgramBuildInfo(program, deviceId, CL_PROGRAM_BUILD_LOG, 0, nullptr, &log_size);
    // Allocate memory for the log
    std::string log;
    log.resize(log_size / sizeof(std::string::value_type));
    // Get the log
    clGetProgramBuildInfo(program, deviceId, CL_PROGRAM_BUILD_LOG, log_size, log.data(), nullptr);
    throw OpenCLError("Failed to build program executable:\n" + std::string(log), err);
  }

  // Create compute kernel in the program we wish to run
  cl_kernel kernel = clCreateKernel(program, m_kernelName.data(), &err);
  if (kernel == nullptr || err != CL_SUCCESS) {
    throw OpenCLError("Failed to create compute kernel", err);
  }

  cl_mem inputMem = nullptr; // device memory used for the input array
  cl_mem outputMem = nullptr; // device memory used for the output array

  if (m_data) {
    const size_t dataSize = dataTypeToSize(m_data->type) * m_data->size;
    // Create the input and output arrays in device memory for our calculation
    inputMem = clCreateBuffer(context, CL_MEM_READ_ONLY, dataSize, nullptr, nullptr);
    outputMem = clCreateBuffer(context, CL_MEM_WRITE_ONLY, dataSize, nullptr, nullptr);
    if (inputMem == nullptr || outputMem == nullptr) {
      throw OpenCLError("Failed to allocate device memory", err);
    }

    // Write our data set into the inputMem array in device memory
    err = clEnqueueWriteBuffer(commands, inputMem, CL_TRUE, 0, dataSize, m_data->input, 0, nullptr, nullptr);
    if (err != CL_SUCCESS) {
      throw OpenCLError("Failed to write to source array", err);
    }

    // Set the arguments to our compute kernel
    err = 0;
    err = clSetKernelArg(kernel, 0, sizeof(cl_mem), &inputMem);
    err |= clSetKernelArg(kernel, 1, sizeof(cl_mem), &outputMem);
    if (err != CL_SUCCESS) {
      throw OpenCLError("Failed to set kernel arguments", err);
    }
  }

  // Execute the kernel over the entire range of our 1d inputMem data set
  // using the maximum number of work group items for this device
  cl_event event = nullptr;
  err = clEnqueueNDRangeKernel(commands, kernel, m_globalWorkSizes.size(), nullptr, m_globalWorkSizes.data(),
                               m_localWorkSizes.empty() ? nullptr : m_localWorkSizes.data(), 0, nullptr,
                               &event);
  if (err != CL_SUCCESS) {
    throw OpenCLError("Failed to execute kernel", err);
  }

  // Wait for the command commands to get serviced before reading back results
  clFinish(commands);

  if (m_isProfilingEnabled) {
    cl_ulong start = 0;
    cl_ulong end = 0;
    err = clGetEventProfilingInfo(event, CL_PROFILING_COMMAND_START, sizeof(start), &start, nullptr);
    err = clGetEventProfilingInfo(event, CL_PROFILING_COMMAND_END, sizeof(end), &end, nullptr);
    if (err != CL_SUCCESS) {
      throw OpenCLError("Failed to get event profiling info", err);
    }
    m_executionTime = std::chrono::nanoseconds(end - start);
  }

  if (m_data) {
    const size_t dataSize = dataTypeToSize(m_data->type) * m_data->size;
    // Read back the results from the device to verify the outputMem
    err = clEnqueueReadBuffer(commands, outputMem, CL_TRUE, 0, dataSize, m_data->output, 0, nullptr, nullptr);
    if (err != CL_SUCCESS) {
      throw OpenCLError("Failed to read output array", err);
    }

    // Shutdown and cleanup
    clReleaseMemObject(inputMem);
    clReleaseMemObject(outputMem);
  }

  clReleaseEvent(event);
  clReleaseKernel(kernel);
  clReleaseProgram(program);
  clReleaseCommandQueue(commands);
  clReleaseContext(context);
  clReleaseDevice(deviceId);
}

std::string Engine::getKernelFilePath() const { return getKernelsDirPath() + m_kernelName + ".cl"; }

std::string Engine::loadKernelSource() const {
  // Open file
  std::ifstream file(getKernelFilePath(), std::ios::in);
  if (!file.is_open()) {
    throw EngineError("Failed to load kernel source");
  }

  // Get source string
  std::ostringstream ss;
  ss << file.rdbuf();
  return ss.str();
}

void Engine::addCompilerOptionDefaultIncludeDirectories() {
  for (const auto& dir : s_defaultIncludeDirectories) {
    addCompilerOptionIncludeDirectory(getKernelsDirPath() + dir);
  }
}

std::string Engine::getKernelsDirPath() {
  const std::string sourceFile = __FILE__;
  std::string kernelsDirPath = sourceFile.substr(0, sourceFile.rfind('/'));
  kernelsDirPath = kernelsDirPath.substr(0, kernelsDirPath.rfind('/'));
  kernelsDirPath += "/kernels/";
  return kernelsDirPath;
}

} // namespace ocl
