# HOW TO USE VULKAN
- Set up is omitted.

## Procedure
1. make ash instance
2. set up glfw
3. choose and check physical device
   - physical device means GPU
4. make logical device
   - logical device

## Words
- instance
  - the instance to operate vulkan. all operation about vulkan is via instance.
- physical device
  - the graphic cards. e.g. GPU
- logical device
  - the virtual memory supplied by the OS
- queue
  - the command for the physical device.
  - We need to know the GPU how many queues has and what function the queue has.