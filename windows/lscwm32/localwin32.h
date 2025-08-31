#ifndef LOCALWIN32_H
#define LOCALWIN32_H

#include <windows.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char name[MAX_PATH];
    DWORD type;
} DriveInfo;

int get_drives(DriveInfo* drives, int max_drives);
bool read_file(const char* path, char** buffer, DWORD* size);
bool write_file(const char* path, const char* data, DWORD size);
HWND find_window_by_title(const char* title);
bool close_window(HWND hwnd);
bool bring_window_to_front(HWND hwnd);

#ifdef __cplusplus
}
#endif

#endif
