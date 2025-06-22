import browser from "webextension-polyfill";
import { OnPortMessageListener } from "./types";
import { onTabMessage } from "./lib/index";

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
      console.log("Received response from native app:", response);
      if (response.type === "SUDACHI") {
        console.log("SUDACHI response received:", response);
      }
    });
  }
};

browser.runtime.onInstalled.addListener(() => {
  console.log("Installed!");
});

onTabMessage((msg) => {
  initializePort();
  if (port) {
    if (msg.type === "COPY_TO_CLIPBOARD") {
      port.postMessage({
        event: msg.type,
        text: msg.text,
      });
    } else if (msg.type === "SEND_SUDACHI") {
      port.postMessage({
        event: msg.type,
        text: msg.text,
        id: msg.id,
      });
    }
  }
});
