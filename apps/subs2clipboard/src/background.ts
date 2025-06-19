import browser from "webextension-polyfill";

console.log("Background script loaded!");

let enabled = false;
let port: browser.Runtime.Port | null = null;

const updateIcon = (tabId: number | undefined) => {
    const iconPath = enabled ? "icon-32.png" : "icon-gray-32.png";
    console.log(`Updating icon to: ${iconPath} for tab: ${tabId}`);
    browser.browserAction.setIcon({ path: iconPath, tabId });
}

browser.runtime.onInstalled.addListener(() => {
  console.log("Installed!");
});

browser.runtime.onMessage.addListener((msg: any) => {
    if (msg.type === "COPY_TO_CLIPBOARD") {
        if (!port) {
            port = browser.runtime.connectNative("subs2clipboard");
            port.onDisconnect.addListener(() => {
                console.log("Native port disconnected");
                port = null;
            });
            port.onMessage.addListener((response) => {
                console.log("Received response from native app:", response);
            });
        }
        port.postMessage({
            text: msg.text
        });
    }
});

browser.browserAction.onClicked.addListener((tab) => {
    console.log("Browser action clicked", tab);
    console.log("Extension background script is working!");
    console.log("Current enabled state:", enabled);
    enabled = !enabled;
    console.log("New enabled state:", enabled);
    updateIcon(tab.id);
    browser.tabs.sendMessage(tab.id ?? chrome.tabs.TAB_ID_NONE, { type: "TOGGLE_SUBS", enabled });
});

browser.tabs.onActivated.addListener((activeInfo) => {
    updateIcon(activeInfo.tabId);
});