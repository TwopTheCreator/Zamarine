#include <xchemoxide.h>
#include <stdio.h>

int main(int argc, char* argv[]) {
    // Initialize the library
    xchem_ctx* ctx = xchem_init("XChemOxide Example");
    if (!ctx) {
        fprintf(stderr, "Failed to initialize XChemOxide\n");
        return 1;
    }

    // Create a window
    xchem_window* window = xchem_create_window(ctx, "XChemOxide Example", 800, 600, true);
    if (!window) {
        fprintf(stderr, "Failed to create window\n");
        xchem_shutdown(ctx);
        return 1;
    }

    // Create a surface
    xchem_surface* surface = xchem_create_surface(ctx, 256, 256, XCHEM_PIXELFORMAT_RGBA8888);
    if (surface) {
        // Lock surface to modify pixels
        uint32_t* pixels = (uint32_t*)xchem_lock_surface(surface);
        if (pixels) {
            // Draw a simple gradient
            for (int y = 0; y < 256; y++) {
                for (int x = 0; x < 256; x++) {
                    uint8_t r = x;
                    uint8_t g = y;
                    uint8_t b = (x + y) / 2;
                    pixels[y * 256 + x] = (0xFF << 24) | (b << 16) | (g << 8) | r;
                }
            }
            xchem_unlock_surface(surface);
        }
    }

    // Main loop
    bool running = true;
    xchem_event event;
    xchem_rect rect = {100, 100, 200, 150};
    float hue = 0.0f;

    while (running) {
        // Process events
        while (xchem_poll_event(ctx, &event)) {
            switch (event.type) {
                case XCHEM_EVENT_QUIT:
                    running = false;
                    break;
                case XCHEM_EVENT_WINDOW_RESIZED:
                    printf("Window resized to %dx%d\n", 
                           event.data.window_resized.width, 
                           event.data.window_resized.height);
                    break;
                default:
                    break;
            }
        }

        // Update
        hue += 1.0f;
        if (hue >= 360.0f) hue = 0.0f;
        
        // Convert HSV to RGB
        float c = 1.0f;
        float x = c * (1.0f - fabsf(fmodf(hue / 60.0f, 2.0f) - 1.0f));
        float m = 0.2f;
        float r, g, b;
        
        if (hue < 60) { r = c; g = x; b = 0; }
        else if (hue < 120) { r = x; g = c; b = 0; }
        else if (hue < 180) { r = 0; g = c; b = x; }
        else if (hue < 240) { r = 0; g = x; b = c; }
        else if (hue < 300) { r = x; g = 0; b = c; }
        else { r = c; g = 0; b = x; }
        
        xchem_color color = {r, g, b, 1.0f};
        
        // Draw
        xchem_clear(window, (xchem_color){0.1f, 0.1f, 0.1f, 1.0f});
        
        // Draw a rotating rectangle
        xchem_set_draw_color(window, color);
        xchem_fill_rect(window, &rect);
        
        // Draw the surface if it was created
        if (surface) {
            xchem_rect src_rect = {0, 0, 256, 256};
            xchem_rect dst_rect = {300, 200, 256, 256};
            xchem_draw_surface(window, surface, &src_rect, &dst_rect);
        }
        
        xchem_present(window);
    }

    // Cleanup
    if (surface) {
        xchem_destroy_surface(surface);
    }
    xchem_destroy_window(window);
    xchem_shutdown(ctx);
    
    return 0;
}
