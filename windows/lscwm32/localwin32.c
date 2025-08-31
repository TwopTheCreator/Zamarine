#include "localwin32.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <tchar.h>

int get_drives(DriveInfo* drives, int max_drives) {
    DWORD mask = GetLogicalDrives();
    int count = 0;
    for (int i = 0; i < 26 && count < max_drives; i++) {
        if (mask & (1 << i)) {
            snprintf(drives[count].name, MAX_PATH, "%c:\\", 'A' + i);
            drives[count].type = GetDriveTypeA(drives[count].name);
            count++;
        }
    }
    return count;
}

bool read_file(const char* path, char** buffer, DWORD* size) {
    HANDLE h = CreateFileA(path, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (h == INVALID_HANDLE_VALUE) return false;
    *size = GetFileSize(h, NULL);
    *buffer = malloc(*size);
    if (!*buffer) {
        CloseHandle(h);
        return false;
    }
    DWORD read;
    bool result = ReadFile(h, *buffer, *size, &read, NULL) && read == *size;
    CloseHandle(h);
    return result;
}

bool write_file(const char* path, const char* data, DWORD size) {
    HANDLE h = CreateFileA(path, GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);
    if (h == INVALID_HANDLE_VALUE) return false;
    DWORD written;
    bool result = WriteFile(h, data, size, &written, NULL) && written == size;
    CloseHandle(h);
    return result;
}

BOOL CALLBACK enum_windows_proc(HWND hwnd, LPARAM lParam) {
    char* title = (char*)lParam;
    char wnd_title[256];
    GetWindowTextA(hwnd, wnd_title, 256);
    if (strstr(wnd_title, title)) {
        *((HWND*)lParam) = hwnd;
        return FALSE;
    }
    return TRUE;
}

HWND find_window_by_title(const char* title) {
    HWND hwnd = 0;
    enum_windows_proc((HWND)title, (LPARAM)&hwnd);
    EnumWindows(enum_windows_proc, (LPARAM)&hwnd);
    return hwnd;
}

bool close_window(HWND hwnd) {
    return PostMessage(hwnd, WM_CLOSE, 0, 0);
}

bool bring_window_to_front(HWND hwnd) {
    if (!IsIconic(hwnd)) return SetForegroundWindow(hwnd);
    ShowWindow(hwnd, SW_RESTORE);
    return SetForegroundWindow(hwnd);
}
