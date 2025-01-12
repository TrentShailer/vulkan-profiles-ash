#include <vulkan/vulkan_core.h>

extern "C"{
    // PFN_vkEnumerateInstanceVersion
    VKAPI_ATTR VkResult VKAPI_CALL vkEnumerateInstanceVersion_MOCK(
        uint32_t*                                   pApiVersion)
    {
        *pApiVersion = VK_MAKE_API_VERSION(0, 1, 2, 198);
        return VK_SUCCESS;
    }

    // PFN_vkGetInstanceProcAddr
    // VKAPI_ATTR PFN_vkVoidFunction VKAPI_CALL vkGetInstanceProcAddr(
    //     VkInstance                                  instance,
    //     const char*                                 pName)
    // {
    //     (PFN_vkGetInstanceProcAddr)vkGetInstanceProcAddr
    // }

    // PFN_vkGetDeviceProcAddr
    // VKAPI_ATTR PFN_vkVoidFunction VKAPI_CALL vkGetDeviceProcAddr(
    //     VkDevice                                    device,
    //     const char*                                 pName)
    // {
    //     (PFN_vkGetDeviceProcAddr)vkGetDeviceProcAddr
    // }

    // PFN_vkEnumerateInstanceExtensionProperties
    VKAPI_ATTR VkResult VKAPI_CALL vkEnumerateInstanceExtensionProperties_MOCK(
        const char*                                 pLayerName,
        uint32_t*                                   pPropertyCount,
        VkExtensionProperties*                      pProperties)
    {
        struct VkExtensionProperties debug_utils = {
            VK_EXT_DEBUG_UTILS_EXTENSION_NAME,
            VK_EXT_DEBUG_UTILS_SPEC_VERSION
        };

        struct VkExtensionProperties surface = {
            VK_KHR_SURFACE_EXTENSION_NAME,
            VK_KHR_SURFACE_SPEC_VERSION
        };

        *pPropertyCount = 2;
        if (pProperties != nullptr) {
            pProperties[0] = debug_utils;
            pProperties[1] = surface;
        }

        return VK_SUCCESS;
    }

    // PFN_vkEnumerateDeviceExtensionProperties
    VKAPI_ATTR VkResult VKAPI_CALL vkEnumerateDeviceExtensionProperties_MOCK(
        VkPhysicalDevice                            physicalDevice,
        const char*                                 pLayerName,
        uint32_t*                                   pPropertyCount,
        VkExtensionProperties*                      pProperties)
    {
        struct VkExtensionProperties sync2 = {
            VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME,
            VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION
        };

        struct VkExtensionProperties atomic_float = {
            VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME,
            VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION
        };


        *pPropertyCount = 2;
        if (pProperties != nullptr) {
            pProperties[0] = sync2;
            pProperties[1] = atomic_float;
        }

        return VK_SUCCESS;
    }

    // PFN_vkGetPhysicalDeviceFeatures2
    VKAPI_ATTR void VKAPI_CALL vkGetPhysicalDeviceFeatures2_MOCK(
        VkPhysicalDevice                            physicalDevice,
        VkPhysicalDeviceFeatures2*                  pFeatures)
    {
        pFeatures->features.shaderFloat64 = VK_TRUE;

        VkBaseOutStructure* next = reinterpret_cast<VkBaseOutStructure*>(pFeatures->pNext);
        while (next != nullptr) {
            switch (next->sType) {
                case VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES:
                    {
                        VkPhysicalDeviceShaderFloat16Int8Features* float16 = reinterpret_cast<VkPhysicalDeviceShaderFloat16Int8Features*>(next);
                        float16->shaderFloat16 = VK_TRUE;
                    }
                    break;

                case VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR:
                    {
                        VkPhysicalDeviceSynchronization2Features* sync2 = reinterpret_cast<VkPhysicalDeviceSynchronization2Features*>(next);
                        sync2->synchronization2 = VK_TRUE;
                    }
                    break;
            }
            next = reinterpret_cast<VkBaseOutStructure*>(next->pNext);
        }
    }

    // PFN_vkGetPhysicalDeviceProperties2
    VKAPI_ATTR void VKAPI_CALL vkGetPhysicalDeviceProperties2_MOCK(
        VkPhysicalDevice                            physicalDevice,
        VkPhysicalDeviceProperties2*                pProperties)
    {
        struct VkPhysicalDeviceProperties properties = {
            VK_MAKE_API_VERSION(0, 1, 2, 198),  // apiVersion
            VK_MAKE_API_VERSION(0, 0, 0, 0),    // driverVersion
            0, // vendorID
            0, // deviceID;
            VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU, // deviceType;
            "Mock Vulkan Device", // deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE];
            {0xcf,0xab, 0x9b,0xdc, 0x74, 0x14, 0x48, 0x61, 0xb7, 0xd6, 0xc4, 0x73, 0x43, 0x98, 0x39, 0x8f}, // pipelineCacheUUID[VK_UUID_SIZE];
        };
        properties.limits.maxImageDimension2D = 32768;
       
        pProperties->properties = properties;
        
        VkBaseOutStructure* next = reinterpret_cast<VkBaseOutStructure*>(pProperties->pNext);
        while (next != nullptr) {
            switch (next->sType) {
                case VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES:
                    {
                        VkPhysicalDeviceSubgroupProperties* prop = reinterpret_cast<VkPhysicalDeviceSubgroupProperties*>(next);
                        prop->subgroupSize = 8;
                        prop->supportedStages = VK_SHADER_STAGE_COMPUTE_BIT | VK_SHADER_STAGE_ALL_GRAPHICS;
                        prop->supportedOperations = VK_SUBGROUP_FEATURE_BASIC_BIT | VK_SUBGROUP_FEATURE_ARITHMETIC_BIT;
                    }
                    break;

                case VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR:
                    {
                        VkPhysicalDevicePushDescriptorPropertiesKHR* prop = reinterpret_cast<VkPhysicalDevicePushDescriptorPropertiesKHR*>(next);
                        prop->maxPushDescriptors = 32;
                    }
                    break;
            }
            next = reinterpret_cast<VkBaseOutStructure*>(next->pNext);
        }
    }

    // PFN_vkGetPhysicalDeviceFormatProperties2
    VKAPI_ATTR void VKAPI_CALL vkGetPhysicalDeviceFormatProperties2_MOCK(
        VkPhysicalDevice                            physicalDevice,
        VkFormat                                    format,
        VkFormatProperties2*                        pFormatProperties)
    {
        switch (format) {
            case VK_FORMAT_R8G8B8A8_UNORM:
                {
                    pFormatProperties->formatProperties.linearTilingFeatures = 
                        VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT | 
                        VK_FORMAT_FEATURE_TRANSFER_DST_BIT | 
                        VK_FORMAT_FEATURE_TRANSFER_SRC_BIT | 
                        VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT;

                    pFormatProperties->formatProperties.optimalTilingFeatures = 
                        VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT |
                        VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT |
                        VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT |
                        VK_FORMAT_FEATURE_TRANSFER_DST_BIT |
                        VK_FORMAT_FEATURE_TRANSFER_SRC_BIT;

                    pFormatProperties->formatProperties.bufferFeatures = 
                        VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT |
                        VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT |
                        VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT;
                    
                    VkBaseOutStructure* next = reinterpret_cast<VkBaseOutStructure*>(pFormatProperties->pNext);
                    while (next != nullptr) {
                        switch (next->sType) {
                            case VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3:
                                {
                                    VkFormatProperties3* prop = reinterpret_cast<VkFormatProperties3*>(next);
                                    prop->linearTilingFeatures = VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT;
                                }
                                break;
                        }
                        next = reinterpret_cast<VkBaseOutStructure*>(next->pNext);
                    }
                }
                break;

            case VK_FORMAT_R16G16B16A16_SFLOAT:
                {
                    pFormatProperties->formatProperties.linearTilingFeatures = 
                        VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT | 
                        VK_FORMAT_FEATURE_TRANSFER_DST_BIT | 
                        VK_FORMAT_FEATURE_TRANSFER_SRC_BIT | 
                        VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT;

                    pFormatProperties->formatProperties.optimalTilingFeatures = 
                        VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT |
                        VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT |
                        VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT |
                        VK_FORMAT_FEATURE_TRANSFER_DST_BIT |
                        VK_FORMAT_FEATURE_TRANSFER_SRC_BIT;

                    pFormatProperties->formatProperties.bufferFeatures = 
                        VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT |
                        VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT |
                        VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT;
                    
                    VkBaseOutStructure* next = reinterpret_cast<VkBaseOutStructure*>(pFormatProperties->pNext);
                    while (next != nullptr) {
                        switch (next->sType) {
                            case VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3:
                                {
                                    VkFormatProperties3* prop = reinterpret_cast<VkFormatProperties3*>(next);
                                    prop->linearTilingFeatures = VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT;
                                }
                                break;
                        }
                        next = reinterpret_cast<VkBaseOutStructure*>(next->pNext);
                    }
                }
                break;
        }
    }

    // PFN_vkGetPhysicalDeviceQueueFamilyProperties2
    VKAPI_ATTR void VKAPI_CALL vkGetPhysicalDeviceQueueFamilyProperties2_MOCK(
        VkPhysicalDevice                            physicalDevice,
        uint32_t*                                   pQueueFamilyPropertyCount,
        VkQueueFamilyProperties2*                   pQueueFamilyProperties)
    {
        *pQueueFamilyPropertyCount = 2;

        if (pQueueFamilyProperties == nullptr) {
            return;
        }

        pQueueFamilyProperties[0].queueFamilyProperties.queueFlags = VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT | VK_QUEUE_TRANSFER_BIT;
        pQueueFamilyProperties[0].queueFamilyProperties.queueCount = 1;
        pQueueFamilyProperties[0].queueFamilyProperties.timestampValidBits = 64;
        pQueueFamilyProperties[0].queueFamilyProperties.minImageTransferGranularity = {1, 1, 1};
        VkBaseOutStructure* next = reinterpret_cast<VkBaseOutStructure*>(pQueueFamilyProperties[0].pNext);
        while (next != nullptr) {
            switch (next->sType) {
                case VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR:
                    {
                        VkQueueFamilyGlobalPriorityPropertiesKHR* prop = reinterpret_cast<VkQueueFamilyGlobalPriorityPropertiesKHR*>(next);
                        prop->priorityCount = 1;
                        prop->priorities[0] = VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR;
                    }
                    break;
            }
            next = reinterpret_cast<VkBaseOutStructure*>(next->pNext);
        }

        pQueueFamilyProperties[1].queueFamilyProperties.queueFlags = VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT | VK_QUEUE_TRANSFER_BIT;
        pQueueFamilyProperties[1].queueFamilyProperties.queueCount = 4;
        pQueueFamilyProperties[1].queueFamilyProperties.timestampValidBits = 64;
        pQueueFamilyProperties[1].queueFamilyProperties.minImageTransferGranularity = {1, 1, 1};
        VkBaseOutStructure* next2 = reinterpret_cast<VkBaseOutStructure*>(pQueueFamilyProperties[1].pNext);
        while (next2 != nullptr) {
            switch (next2->sType) {
                case VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR:
                    {
                        VkQueueFamilyGlobalPriorityPropertiesKHR* prop = reinterpret_cast<VkQueueFamilyGlobalPriorityPropertiesKHR*>(next2);
                        prop->priorityCount = 1;
                        prop->priorities[0] = VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR;
                    }
                    break;
            }
            next2 = reinterpret_cast<VkBaseOutStructure*>(next2->pNext);
        }
    }

    // // PFN_vkCreateInstance
    // VKAPI_ATTR VkResult VKAPI_CALL vkCreateInstance_MOCK(
    //     const VkInstanceCreateInfo*                 pCreateInfo,
    //     const VkAllocationCallbacks*                pAllocator,
    //     VkInstance*                                 pInstance)
    // {
    //     return VK_ERROR_UNKNOWN;
    // }

    // // PFN_vkCreateDevice
    // VKAPI_ATTR VkResult VKAPI_CALL vkCreateDevice_MOCK(
    //     VkPhysicalDevice                            physicalDevice,
    //     const VkDeviceCreateInfo*                   pCreateInfo,
    //     const VkAllocationCallbacks*                pAllocator,
    //     VkDevice*                                   pDevice)
    // {
    //     return VK_ERROR_UNKNOWN;
    // }
}
