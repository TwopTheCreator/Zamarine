document.addEventListener('DOMContentLoaded', () => {
    const dock = document.querySelector('.dock-container');
    const grid = document.querySelector('.dock-grid');
    let hideTimeout;
    let isHovering = false;
    const gridItems = 18; // 3 rows of 6 items
    
    // Create grid items
    for (let i = 0; i < gridItems; i++) {
        const item = document.createElement('div');
        item.className = 'grid-item';
        item.style.opacity = '0';
        item.style.transform = 'translateY(20px)';
        grid.appendChild(item);
    }
    
    // Animate grid items on load
    const items = document.querySelectorAll('.grid-item');
    items.forEach((item, index) => {
        setTimeout(() => {
            item.style.transition = 'opacity 0.5s ease-out, transform 0.5s cubic-bezier(0.2, 0.8, 0.2, 1.2)';
            item.style.opacity = '1';
            item.style.transform = 'translateY(0)';
        }, 50 * (index % 6) + 100 * Math.floor(index / 6));
        
        // Add click effect
        item.addEventListener('click', (e) => {
            e.target.style.transform = 'translateY(-25px) scale(1.15)';
            setTimeout(() => {
                e.target.style.transform = 'translateY(-20px) scale(1.1)';
            }, 150);
        });
    });
    
    // Show/hide dock based on mouse position
    const showDock = () => {
        clearTimeout(hideTimeout);
        if (!dock.classList.contains('visible')) {
            dock.style.transition = 'transform 0.4s cubic-bezier(0.2, 0.8, 0.2, 1.2)';
            dock.classList.add('visible');
        }
    };
    
    const hideDock = () => {
        if (!isHovering) {
            dock.style.transition = 'transform 0.6s cubic-bezier(0.4, 0, 0.2, 1)';
            dock.classList.remove('visible');
        }
    };
    
    document.addEventListener('mousemove', (e) => {
        const mouseY = e.clientY;
        const windowHeight = window.innerHeight;
        const distanceFromBottom = windowHeight - mouseY;
        const showDockThreshold = 80; // pixels from bottom to show dock
        
        if (distanceFromBottom <= showDockThreshold) {
            showDock();
        } else if (dock.classList.contains('visible') && !isHovering) {
            hideTimeout = setTimeout(hideDock, 500);
        }
    });
    
    // Keep dock visible when hovering
    dock.addEventListener('mouseenter', () => {
        isHovering = true;
        clearTimeout(hideTimeout);
        showDock();
    });
    
    dock.addEventListener('mouseleave', () => {
        isHovering = false;
        if (window.innerHeight - event.clientY > 100) {
            hideDock();
        }
    });
    
    // Handle window resize
    window.addEventListener('resize', () => {
        if (window.innerHeight - event.clientY > 100 && !isHovering) {
            clearTimeout(hideTimeout);
            hideDock();
        }
    });
});
