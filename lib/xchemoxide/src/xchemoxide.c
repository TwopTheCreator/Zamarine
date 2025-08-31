#include "xchemoxide.h"
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
#elif defined(__APPLE__)
#include <OpenGL/gl.h>
#else
#include <GL/gl.h>
#endif

struct xchem_ctx {
    bool initialized;
    const char* app_name;
};

struct xchem_surface {
    int width;
    int height;
    xchem_pixelformat format;
    void* pixels;
    int pitch;
    bool locked;
};

struct xchem_window {
    int width;
    int height;
    const char* title;
    bool visible;
    xchem_color draw_color;
};

struct xchem_event {
    xchem_event_type type;
    union {
        struct { int x, y; } mouse_motion;
        struct { int x, y; int button; } mouse_button;
        struct { int width, height; } window_resized;
        int keycode;
    } data;
};

xchem_ctx* xchem_init(const char* app_name) {
    xchem_ctx* ctx = (xchem_ctx*)calloc(1, sizeof(xchem_ctx));
    if (!ctx) return NULL;
    
    ctx->app_name = _strdup(app_name);
    if (!ctx->app_name) {
        free(ctx);
        return NULL;
    }
    ctx->initialized = true;
    return ctx;
}

void xchem_shutdown(xchem_ctx* ctx) {
    if (!ctx) return;
    
    free((void*)ctx->app_name);
    free(ctx);
}

xchem_window* xchem_create_window(xchem_ctx* ctx, const char* title, int width, int height, bool resizable) {
    if (!ctx || !title || width <= 0 || height <= 0) return NULL;
    
    xchem_window* window = (xchem_window*)calloc(1, sizeof(xchem_window));
    if (!window) return NULL;
    
    window->width = width;
    window->height = height;
    window->title = _strdup(title);
    window->visible = false;
    window->draw_color = (xchem_color){1.0f, 1.0f, 1.0f, 1.0f};
    
    return window;
}

void xchem_destroy_window(xchem_window* window) {
    if (!window) return;
    
    free((void*)window->title);
    free(window);
}

void xchem_set_window_title(xchem_window* window, const char* title) {
    if (!window || !title) return;
    
    free((void*)window->title);
    window->title = _strdup(title);
}

void xchem_set_window_size(xchem_window* window, int width, int height) {
    if (!window || width <= 0 || height <= 0) return;
    
    window->width = width;
    window->height = height;
}

void xchem_get_window_size(xchem_window* window, int* width, int* height) {
    if (!window || !width || !height) return;
    
    *width = window->width;
    *height = window->height;
}

xchem_surface* xchem_create_surface(xchem_ctx* ctx, int width, int height, xchem_pixelformat format) {
    if (!ctx || width <= 0 || height <= 0 || format >= XCHEM_PIXELFORMAT_COUNT) {
        return NULL;
    }
    
    xchem_surface* surface = (xchem_surface*)calloc(1, sizeof(xchem_surface));
    if (!surface) return NULL;
    
    size_t bpp = 4; // Default to RGBA
    switch (format) {
        case XCHEM_PIXELFORMAT_RGB888:
        case XCHEM_PIXELFORMAT_BGR888:
            bpp = 3; break;
        case XCHEM_PIXELFORMAT_GRAY8:
            bpp = 1; break;
        default: break;
    }
    
    surface->width = width;
    surface->height = height;
    surface->format = format;
    surface->pitch = width * bpp;
    surface->pixels = calloc((size_t)(width * height * bpp), 1);
    surface->locked = false;
    
    if (!surface->pixels) {
        free(surface);
        return NULL;
    }
    
    return surface;
}

void xchem_destroy_surface(xchem_surface* surface) {
    if (!surface) return;
    
    free(surface->pixels);
    free(surface);
}

void* xchem_lock_surface(xchem_surface* surface) {
    if (!surface) return NULL;
    surface->locked = true;
    return surface->pixels;
}

void xchem_unlock_surface(xchem_surface* surface) {
    if (!surface || !surface->locked) return;
    surface->locked = false;
}

void xchem_clear(xchem_window* window, xchem_color color) {
    if (!window) return;
    glClearColor(color.r, color.g, color.b, color.a);
    glClear(GL_COLOR_BUFFER_BIT);
}

void xchem_present(xchem_window* window) {
    if (!window) return;
}

void xchem_set_draw_color(xchem_window* window, xchem_color color) {
    if (!window) return;
    window->draw_color = color;
    glColor4f(color.r, color.g, color.b, color.a);
}

void xchem_draw_rect(xchem_window* window, const xchem_rect* rect) {
    if (!window || !rect) return;
    glBegin(GL_LINE_LOOP);
    glVertex2i(rect->x, rect->y);
    glVertex2i(rect->x + rect->w, rect->y);
    glVertex2i(rect->x + rect->w, rect->y + rect->h);
    glVertex2i(rect->x, rect->y + rect->h);
    glEnd();
}

void xchem_fill_rect(xchem_window* window, const xchem_rect* rect) {
    if (!window || !rect) return;
        glBegin(GL_QUADS);
    glVertex2i(rect->x, rect->y);
    glVertex2i(rect->x + rect->w, rect->y);
    glVertex2i(rect->x + rect->w, rect->y + rect->h);
    glVertex2i(rect->x, rect->y + rect->h);
    glEnd();
}

void xchem_draw_surface(xchem_window* window, xchem_surface* surface, const xchem_rect* src, const xchem_rect* dst) {
    if (!window || !surface || !dst) return;
    
    glEnable(GL_TEXTURE_2D);
    glDisable(GL_TEXTURE_2D);
}

bool xchem_poll_event(xchem_ctx* ctx, xchem_event* event) {
    if (!ctx || !event) return false;
    
    event->type = XCHEM_EVENT_NONE;
    return false;
}

void xchem_cleanup(xchem_ctx* ctx) {
    if (!ctx) return;

    
    xchem_shutdown(ctx);
}
