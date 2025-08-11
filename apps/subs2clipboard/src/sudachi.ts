import { onRuntimeMessage, sendRuntimeMessage } from "./lib/content-helper";
import { log } from "./content-debug";
import { Metadata } from "@nicolasnewman/kanji-bank-types";

const style = document.createElement('style');
style.textContent = `
  .kanji {
    color: #FF00AA;
  }
  .vocabulary {
    color: #AA00FF;
  }
  .subs2clipboard-popup-parent {
    position: relative;
}
  .subs2clipboard-popup {
    position: fixed;
    background-color: #fff;
    border: 1px solid #aaa;
    padding: 5px;
    z-index: 1000;
    color: #000;
    font-weight: normal;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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

const decodeBase64UTF8 = (str: string): string => {
  const utf8BytesDecoded = Uint8Array.from(atob(str), c => c.charCodeAt(0));
  const textDecoder = new TextDecoder();
  return textDecoder.decode(utf8BytesDecoded);
}

const createHoverPopup = (spanEl: HTMLElement) => {
  // spanEl.classList.add("subs2clipboard-popup-parent");
  const metadata = JSON.parse(decodeBase64UTF8(spanEl.getAttribute("data-metadata") || "")) as Metadata;
  const source = spanEl.getAttribute("data-source") || "unknown";
  const meaning = spanEl.getAttribute("data-meaning") || "";
  const reading = spanEl.getAttribute("data-reading");
  console.log(metadata);

  const { left, bottom } = spanEl.getBoundingClientRect();
  const popup = document.createElement("div");
  popup.style.left = `${left}px`;
  popup.style.top = `${bottom}px`;
  popup.classList.add("subs2clipboard-popup");
  popup.style.borderColor = getComputedStyle(spanEl).color || "#aaa";
  popup.innerHTML = `
  <div>${spanEl.innerText}</div>
  <div><strong>Meaning:</strong> ${meaning}</div>
  `;
  if (reading) {
    popup.innerHTML += `
      <div><strong>Reading:</strong> ${reading}</div>
    `;
  }
  popup.innerHTML += `
    <div><strong>Source:</strong> ${source}</div>
  `;
  if (source === "wanikani") {
    if (metadata?.vocabularyData) {
      popup.innerHTML += `
        <div><strong>Part of Speech:</strong> ${metadata.vocabularyData.partsOfSpeech.join(", ")}</div>
      `;
    }
    if (metadata?.kanjiData) {
      if (metadata.kanjiData?.onyomiReadings.length > 0) {
        popup.innerHTML += `
          <div><strong>Onyomi Reading:</strong> ${metadata.kanjiData.onyomiReadings.join(", ")}</div>
        `;
      }
      if (metadata.kanjiData?.kunyomiReadings.length > 0) {
        popup.innerHTML += `
          <div><strong>Kunyomi Reading:</strong> ${metadata.kanjiData.kunyomiReadings.join(", ")}</div>
        `;
      }
      if (metadata.kanjiData?.nanoriReadings.length > 0) {
        popup.innerHTML += `
          <div><strong>Nanori Reading:</strong> ${metadata.kanjiData.nanoriReadings.join(", ")}</div>
        `;
      }
    }
    popup.innerHTML += `
      <a href="${metadata?.url}">See more</a>
    `;
  };
  spanEl.appendChild(popup);

  return popup;
}

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
          log("Sending block text to Sudachi:", text, id);
          sendRuntimeMessage({
            type: "SEND_SUDACHI",
            text,
            id,
            tabId: msg.tabId
          })
        }
      });
    });

    const processedDivTextNodes = new WeakSet<Node>();
    const validDivNodes: Node[] = [];
    document.querySelectorAll("div").forEach((el) => {
      const text = (el as HTMLElement).innerHTML || "";
      if (containsJapanese(text)) {
        const walker = document.createTreeWalker(
          el,
          NodeFilter.SHOW_TEXT,
          null
        );

        let node;
        while ((node = walker.nextNode()) !== null) {
          if (containsJapanese(node.textContent || "") && !processedDivTextNodes.has(node)) {
            processedDivTextNodes.add(node);
            validDivNodes.push(node);
          }
        }
      }
    });
    log("Valid div text nodes:", validDivNodes.length);
    validDivNodes.forEach((node) => {
      const span = document.createElement("span");
      span.textContent = node.textContent;
      node.parentNode?.replaceChild(span, node);

      if (span.textContent) {
        const id = `sudachi-${nodeIdCounter++}`;
        elementMap[id] = span as HTMLElement;
        log("Sending div text to Sudachi:", span.textContent, id);
        sendRuntimeMessage({
          type: "SEND_SUDACHI",
          text: span.textContent,
          id,
          tabId: msg.tabId
        })
      }
    });
  } else if (msg.type === "UPDATE_SUDACHI") {
    const { text, id } = msg;
    log("Received Sudachi response:", text, id);
    log("Element map:", elementMap[id].innerHTML);
    elementMap[id].innerHTML = text;
    elementMap[id].querySelectorAll(`span[data-source="wanikani"]`).forEach((el) => {
      let timeout: number | undefined = undefined;
      let popup: HTMLDivElement | null = null;
      el.addEventListener("mouseenter", (hoverEl) => {
        timeout = setTimeout(() => {
          log("hovered element:", hoverEl);
          popup = createHoverPopup(hoverEl.target as HTMLElement);
        }, 1000);
      });
      el.addEventListener("mouseleave", () => {
        clearTimeout(timeout);
        setTimeout(() => {
          popup?.remove();
        }, 500);
      });
    });

  }
});
