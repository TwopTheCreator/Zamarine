#ifndef XCHEMOXIDE_H
#define XCHEMOXIDE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct xchem_ctx xchem_ctx;
typedef struct xchem_surface xchem_surface;
typedef struct xchem_window xchem_window;
typedef struct xchem_event xchem_event;

typedef enum {
    XCHEM_PIXELFORMAT_RGBA8888,
    XCHEM_PIXELFORMAT_BGRA8888,
    XCHEM_PIXELFORMAT_RGB888,
    XCHEM_PIXELFORMAT_BGR888,
    XCHEM_PIXELFORMAT_GRAY8,
    XCHEM_PIXELFORMAT_COUNT
} xchem_pixelformat;

typedef enum {
    XCHEM_BLENDMODE_NONE,
    XCHEM_BLENDMODE_BLEND,
    XCHEM_BLENDMODE_ADD,
    XCHEM_BLENDMODE_MODULATE
} xchem_blendmode;

typedef enum {
    XCHEM_EVENT_NONE,
    XCHEM_EVENT_QUIT,
    XCHEM_EVENT_KEY_DOWN,
    XCHEM_EVENT_KEY_UP,
    XCHEM_EVENT_MOUSE_MOTION,
    XCHEM_EVENT_MOUSE_BUTTON_DOWN,
    XCHEM_EVENT_MOUSE_BUTTON_UP,
    XCHEM_EVENT_WINDOW_RESIZED
} xchem_event_type;

typedef struct {
    float r, g, b, a;
} xchem_color;

typedef struct {
    int x, y, w, h;
} xchem_rect;

// Core functions
xchem_ctx* xchem_init(const char* app_name);
void xchem_shutdown(xchem_ctx* ctx);

// Window management
xchem_window* xchem_create_window(xchem_ctx* ctx, const char* title, int width, int height, bool resizable);
void xchem_destroy_window(xchem_window* window);
void xchem_set_window_title(xchem_window* window, const char* title);
void xchem_set_window_size(xchem_window* window, int width, int height);
void xchem_get_window_size(xchem_window* window, int* width, int* height);

// Surface management
xchem_surface* xchem_create_surface(xchem_ctx* ctx, int width, int height, xchem_pixelformat format);
xchem_surface* xchem_load_surface_from_file(xchem_ctx* ctx, const char* filename);
void xchem_destroy_surface(xchem_surface* surface);
void* xchem_lock_surface(xchem_surface* surface);
void xchem_unlock_surface(xchem_surface* surface);

// Drawing functions
void xchem_clear(xchem_window* window, xchem_color color);
void xchem_present(xchem_window* window);
void xchem_set_draw_color(xchem_window* window, xchem_color color);
void xchem_draw_rect(xchem_window* window, const xchem_rect* rect);
void xchem_fill_rect(xchem_window* window, const xchem_rect* rect);
void xchem_draw_surface(xchem_window* window, xchem_surface* surface, const xchem_rect* src, const xchem_rect* dst);

// Event handling
bool xchem_poll_event(xchem_ctx* ctx, xchem_event* event);

// Cleanup
void xchem_cleanup(xchem_ctx* ctx);

#ifdef __cplusplus
}
#endif

#endif // XCHEMOXIDE_H
