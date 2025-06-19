import browser, { Browser } from "webextension-polyfill";
import { OnBrowserMessageListener, OnPortMessageListener } from "./types";

console.log("Background script loaded!");

let enabled = false;
let port: browser.Runtime.Port | null = null;

const updateIcon = (tabId: number | undefined) => {
    const iconPath = enabled ? "icon-32.png" : "icon-gray-32.png";
    console.log(`Updating icon to: ${iconPath} for tab: ${tabId}`);
    browser.browserAction.setIcon({ path: iconPath, tabId });
}

const initializePort = () => {
    if (!port) {
        port = browser.runtime.connectNative("subs2clipboard");
        port.onDisconnect.addListener(() => {
            console.log("Native port disconnected");
            port = null;
        });
        (port.onMessage as OnPortMessageListener).addListener((response) => {
            console.log("Received response from native app:", response);
            if (response.type === 'SUDACHI') {
                console.log("SUDACHI response received:", response);
            }
        });
    }
}

browser.runtime.onInstalled.addListener(() => {
  console.log("Installed!");
});

(browser.runtime.onMessage as OnBrowserMessageListener).addListener((msg) => {
    initializePort();
    if (port) {
        if (msg.type === "COPY_TO_CLIPBOARD") {
            port.postMessage({
                event: msg.type,
                text: msg.text
            });
        } else if (msg.type === "SUDACHI") {
            port.postMessage({
                event: msg.type,
                text: msg.text,
                id: msg.id
            });
        }
    }
});

browser.browserAction.onClicked.addListener((tab) => {
    console.log("Browser action clicked", tab);
    console.log("Extension background script is working!");
    console.log("Current enabled state:", enabled);
    enabled = !enabled;
    console.log("New enabled state:", enabled);
    updateIcon(tab.id);
    browser.tabs.sendMessage(tab.id ?? browser.tabs.TAB_ID_NONE, { type: "TOGGLE_SUBS", enabled });
});

browser.tabs.onActivated.addListener((activeInfo) => {
    updateIcon(activeInfo.tabId);
});