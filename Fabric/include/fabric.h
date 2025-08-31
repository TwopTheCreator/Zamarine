#ifndef FABRIC_H
#define FABRIC_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef _WIN32
    #ifdef FABRIC_EXPORTS
        #define FABRIC_API __declspec(dllexport)
    #else
        #define FABRIC_API __declspec(dllimport)
    #endif
#else
    #define FABRIC_API __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

FABRIC_API bool fabric_init();
FABRIC_API bool fabric_index_data(const char* key, const uint8_t* data, size_t length);
FABRIC_API bool fabric_search(const char* query, char** result);
FABRIC_API void fabric_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif
