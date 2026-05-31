// Sidebar persistence logic (Leapfrog 2030)
document.addEventListener('DOMContentLoaded', () => {
    const toc = document.querySelector('.toc-sidebar');
    if (!toc) return;

    // Load persisted state from localStorage
    const isPinned = localStorage.getItem('toc-pinned') === 'true';
    if (isPinned) {
        toc.classList.add('pinned');
    }

    const toggleBtn = document.querySelector('.toc-toggle');
    if (toggleBtn) {
        toggleBtn.addEventListener('click', () => {
            const pinned = toc.classList.toggle('pinned');
            localStorage.setItem('toc-pinned', pinned);
        });
    }
});