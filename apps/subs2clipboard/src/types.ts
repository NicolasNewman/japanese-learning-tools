import browser from "webextension-polyfill";

type PortEventPayload = 
  | { type: "SUDACHI"; text: string, id: string };

type MessageEventPayload = 
  | { type: "COPY_TO_CLIPBOARD"; text: string }
  | { type: "SUDACHI"; text: string, id: string };
  
export type OnBrowserMessageListener = browser.Events.Event<(
        message: MessageEventPayload,
        sender: browser.Runtime.MessageSender,
        sendResponse: (response: unknown) => void,
    ) => void>;

export type OnPortMessageListener = browser.Events.Event<(message: PortEventPayload, port: browser.Runtime.Port) => void>