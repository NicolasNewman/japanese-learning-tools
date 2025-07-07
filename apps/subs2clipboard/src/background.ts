import browser from "webextension-polyfill";
import { OnPortMessageListener } from "./types";
import { onRuntimeMessage } from "./lib/content-helper";
import { sendMessageToTab } from "./lib/background-helper";

console.log("Background script loaded!");

let port: browser.Runtime.Port | null = null;

const initializePort = () => {
  if (!port) {
    port = browser.runtime.connectNative("subs2clipboard_native_messenger");
    port.onDisconnect.addListener(() => {
      console.log("Native port disconnected");
      console.log(port?.error);
      port = null;
    });
    (port.onMessage as OnPortMessageListener).addListener((response) => {
      if (response.type === "RECEIVE_SUDACHI") {
        console.log("SUDACHI response received:", response);
        sendMessageToTab(response.tabId, { type: "UPDATE_SUDACHI", text: response.text, id: response.id });
      }
    });
  }
};

browser.runtime.onInstalled.addListener(() => {
  console.log("Installed!");
});

onRuntimeMessage((msg) => {
  initializePort();
  if (port) {
    if (msg.type === "COPY_TO_CLIPBOARD") {
      port.postMessage({
        event: msg.type,
        text: msg.text,
      });
    } else if (msg.type === "SEND_SUDACHI") {
      console.log("Sending SUDACHI message to native app:", msg);
      port.postMessage({
        event: msg.type,
        text: msg.text,
        id: msg.id,
        tabId: msg.tabId,
      });
    }
  }
});
