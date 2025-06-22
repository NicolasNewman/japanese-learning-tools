import browser from "webextension-polyfill";

const copySubsCheckbox = document.getElementById("copyOnClick") as HTMLInputElement;
browser.storage.session.get("copyOnClick").then((item) => {
    copySubsCheckbox.checked = (item.copyOnClick as boolean) ?? false;
});
copySubsCheckbox.addEventListener("change", (e) => {
    const enabled = (e.target as HTMLInputElement).checked;
    browser.storage.session.set({ copyOnClick: enabled });
    console.log("Checkbox changed:", enabled);
    browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
        browser.tabs.sendMessage(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "TOGGLE_SUBS", enabled });
    });
});

const modeSpan = document.getElementById("mode") as HTMLSpanElement;
browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
    browser.tabs.sendMessage(tabs[0].id ?? browser.tabs.TAB_ID_NONE, { type: "GET_SUPPORTED_SERVICE" }).then((response) => {
        modeSpan.textContent = (response as string) || 'None';
    });
});