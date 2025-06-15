import browser from "webextension-polyfill";

let enabled = false;

const updateIcon = (tabId: number | undefined) => {
    const iconPath = enabled ? "icon-32.jpg" : "icon-gray-32.png";
    browser.browserAction.setIcon({ path: iconPath, tabId });
}

browser.browserAction.onClicked.addListener((tab) => {
    enabled = !enabled;
    updateIcon(tab.id);
    browser.tabs.sendMessage(tab.id ?? chrome.tabs.TAB_ID_NONE, { type: "TOGGLE_SUBS", enabled });
});

browser.tabs.onActivated.addListener((activeInfo) => {
    updateIcon(activeInfo.tabId);
});