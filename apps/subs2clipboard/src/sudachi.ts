import browser from "webextension-polyfill";
import { onTabMessage } from "./lib/index";

onTabMessage((msg) => {
  if (msg.type === "RUN_SUDACHI") {
    browser.runtime.sendMessage({
      type: msg.type,
      text: "",
      id: "1",
    });
  }
});
