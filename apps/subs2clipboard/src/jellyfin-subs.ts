import browser from "webextension-polyfill";

/* eslint-disable no-inner-declarations */
if (document.querySelector('body.libraryDocument')) {
    let observer: MutationObserver | null = null;
    let lastSubs = "";

    function sendToClipboard(text: string) {
        browser.runtime.sendMessage({
            type: "COPY_TO_CLIPBOARD",
            text: text
        });
    }
 
    function waitForSubtitlesElement(selector: string, callback: (el: Node) => void) {
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
    }

    function startSubsCopy() {
        if (observer) return;
        waitForSubtitlesElement('.videoSubtitles', (subEl: Node) => {
            observer = new MutationObserver((mutationsList) => {
                const subs = (mutationsList[0].target as HTMLElement).innerText;
                if (subs !== lastSubs) {
                    console.log("Subtitles changed:", subs);
                    sendToClipboard(subs);
                    // navigator.clipboard.writeText(subs);
                    lastSubs = subs;
                }
            });
            observer.observe(subEl, { characterData: true, childList: true, subtree: true });
        });
    }

    function stopSubsCopy() {
        if (observer) {
            observer.disconnect();
            observer = null;
        }
    }

    browser.runtime.onMessage.addListener((msg) => {
        if (msg.type === "TOGGLE_SUBS") {
            if (msg.enabled) {
                startSubsCopy();
            } else {
                stopSubsCopy();
            }
        }
    });
}