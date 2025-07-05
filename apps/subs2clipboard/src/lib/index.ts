type Service = "Jellyfin";
type ServiceSubtitleSelector = {
  service: Service;
  selector: string;
}

export const getSupportedSubtitleSelector = (): ServiceSubtitleSelector | null => {
  const isJellyfin = document.querySelector("body.libraryDocument");
  if (isJellyfin) {
    return {
      service: "Jellyfin",
      selector: ".videoSubtitles",
    };
  }
  return null;
};

export const waitForSupportedService = (timeout = 10000) => {
  return new Promise<ServiceSubtitleSelector>((resolve, reject) => {
    const service = getSupportedSubtitleSelector();
    if (service) {
      resolve(service);
      return;
    }

    const observer = new MutationObserver(() => {
      const service = getSupportedSubtitleSelector();
      if (service) {
        observer.disconnect();
        clearTimeout(timer);
        resolve(service);
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true
    });

    const timer = setTimeout(() => {
      observer.disconnect();
      reject('No supported service found within the timeout period');
    }, timeout);
  });
}