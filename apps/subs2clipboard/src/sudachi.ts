import browser from "webextension-polyfill";
import { onRuntimeMessage, sendRuntimeMessage } from "./lib/content-helper";

const style = document.createElement('style');
style.textContent = `
  /* Your CSS here */
  .kanji {
    color: #FF00AA;
  }
  .vocabulary {
    color: #AA00FF;
  }
`;
document.head.appendChild(style);

const IGNORED_TAGS = new Set([
  "SCRIPT",
  "STYLE",
  "NOSCRIPT",
  "IFRAME",
  "OBJECT",
]);

const BLOCK_TAGS = ["P", "SECTION", "ARTICLE", "BLOCKQUOTE"];

// const getLiTextExcludingNestedLists = (el: Element): string => {
//   let text = "";
//   for (const node of el.childNodes) {
//     if (node.nodeType === Node.TEXT_NODE) {
//       text += node.innerHTML;
//     } else if (
//       node.nodeType === Node.ELEMENT_NODE &&
//       !["UL", "OL"].includes((node as Element).tagName)
//     ) {
//       text += getLiTextExcludingNestedLists(node as Element);
//     }
//     // Skip UL/OL and their children
//   }
//   return text;
// };

const containsJapanese = (text: string): boolean =>
  /[\u3000-\u303F\u3040-\u309F\u30A0-\u30FF\u4E00-\u9FFF]/.test(text);

let nodeIdCounter = 0;
const elementMap: Record<string, HTMLElement> = {};

onRuntimeMessage((msg) => {
  if (msg.type === "RUN_SUDACHI") {
    // 1. Handle block tags (excluding lists)
    BLOCK_TAGS.forEach((tag) => {
      document.querySelectorAll(tag).forEach((el) => {
        // Ignore if inside an ignored tag
        let parent = el.parentElement;
        while (parent) {
          if (IGNORED_TAGS.has(parent.tagName)) return;
          parent = parent.parentElement;
        }
        const text = (el as HTMLElement).innerHTML || "";
        if (containsJapanese(text)) {
          const id = `sudachi-${nodeIdCounter++}`;
          elementMap[id] = el as HTMLElement;
          console.log("Sending block text to Sudachi:", text, id);
          sendRuntimeMessage({
            type: "SEND_SUDACHI",
            text,
            id,
            tabId: msg.tabId
          })
        }
      });
    });

    // // 2. Handle lists: each <li> as a unit, only its own text (not nested lists)
    // document.querySelectorAll("li").forEach((li: Element) => {
    //   // Ignore if inside an ignored tag
    //   let parent = li.parentElement;
    //   while (parent) {
    //     if (IGNORED_TAGS.has(parent.tagName)) return;
    //     parent = parent.parentElement;
    //   }
    //   const text = getLiTextExcludingNestedLists(li).trim();
    //   if (containsJapanese(text)) {
    //     const id = `sudachi-${nodeIdCounter++}`;
    //     console.log("Sending li text to Sudachi:", text, id);
    //     browser.runtime.sendMessage({
    //       type: "SEND_SUDACHI",
    //       text,
    //       id,
    //     });
    //   }
    // });
  } else if (msg.type === "UPDATE_SUDACHI") {
    const { text, id } = msg;
    console.log("Received Sudachi response:", text, id);
    console.log("Element map:", elementMap[id]);
    elementMap[id].innerHTML = text;
  }
});
