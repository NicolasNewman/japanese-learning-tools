export const getSupportedSubtitleSelector = () => {
    const isJellyfin = document.querySelector('body.libraryDocument');
    if (isJellyfin) {
        return {
            service: 'Jellyfin',
            selector: '.videoSubtitles'
        };
    }
    return null;
}