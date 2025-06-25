import browser from "webextension-polyfill";
import { getSupportedSubtitleSelector } from "./lib/index";
import { onRuntimeMessage } from "./lib/content-helper";

const subtitleSelector = getSupportedSubtitleSelector();
if (subtitleSelector) {
  let observer: MutationObserver | null = null;
  let lastSubs = "";

  const sendToClipboard = (text: string) => {
    browser.runtime.sendMessage({
      type: "COPY_TO_CLIPBOARD",
      text: text,
    });
  };

  const waitForSubtitlesElement = (
    selector: string,
    callback: (el: Node) => void
  ) => {
    const el = document.querySelector(selector);
    if (el) {
      callback(el);
      return;
    }
    const observer = new MutationObserver(() => {
      const el = document.querySelector(selector);
      if (el) {
        observer.disconnect();
        callback(el);
      }
    });
    observer.observe(document.body, { childList: true, subtree: true });
  };

  const startSubsCopy = () => {
    if (observer) return;
    waitForSubtitlesElement(subtitleSelector.selector, (subEl: Node) => {
      observer = new MutationObserver((mutationsList) => {
        const subs = (mutationsList[0].target as HTMLElement).innerText;
        if (subs !== lastSubs) {
          console.log("Subtitles changed:", subs);
          sendToClipboard(subs);
          lastSubs = subs;
        }
      });
      observer.observe(subEl, {
        characterData: true,
        childList: true,
        subtree: true,
      });
    });
  };

  const stopSubsCopy = () => {
    if (observer) {
      observer.disconnect();
      observer = null;
    }
  };
  
  onRuntimeMessage((msg, _sender, sendResponse) => {
    if (msg.type === "TOGGLE_SUBS") {
      if (msg.enabled) {
        startSubsCopy();
      } else {
        stopSubsCopy();
      }
    } else if (msg.type === "GET_SUPPORTED_SERVICE") {
      sendResponse(subtitleSelector.service);
    }
  });
}
