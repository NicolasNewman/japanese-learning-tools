import browser from "webextension-polyfill";

export const getSupportedSubtitleSelector = () => {
  const isJellyfin = document.querySelector("body.libraryDocument");
  if (isJellyfin) {
    return {
      service: "Jellyfin",
      selector: ".videoSubtitles",
    };
  }
  return null;
};

type SendMessageToTabEvents = {
  TOGGLE_SUBS: {
    message: {
      type: "TOGGLE_SUBS";
      enabled: boolean;
    };
    response: null;
  };
  GET_SUPPORTED_SERVICE: {
    message: {
      type: "GET_SUPPORTED_SERVICE";
    };
    response: string | null;
  };
  RUN_SUDACHI: {
    message: {
      type: "RUN_SUDACHI";
    };
    response: null;
  };
};
export const sendMessageToTab = browser.tabs.sendMessage as <
  T extends keyof SendMessageToTabEvents
>(
  tabId: number,
  message: SendMessageToTabEvents[T]["message"]
) => Promise<SendMessageToTabEvents[T]["response"]>;

type OnTabMessageEvents = {
  TOGGLE_SUBS: {
    message: {
      type: "TOGGLE_SUBS";
      enabled: boolean;
    };
    response: null;
  };
  COPY_TO_CLIPBOARD: {
    message: {
        type: "COPY_TO_CLIPBOARD";
        text: string;
    };
    response: null;
  }
  GET_SUPPORTED_SERVICE: {
    message: {
      type: "GET_SUPPORTED_SERVICE";
    };
    response: string
  };
  RUN_SUDACHI: {
    message: {
      type: "RUN_SUDACHI";
    };
    response: null;
  };
  SEND_SUDACHI: {
    message: {
      type: "SEND_SUDACHI";
      text: string;
      id: string;
    };
    response: null;
  }
};
type OnTabMessageCallback<T extends keyof OnTabMessageEvents> = (
  msg: OnTabMessageEvents[T]["message"],
  sender: browser.Runtime.MessageSender,
  sendResponse: (response: OnTabMessageEvents[T]["response"]) => void
) => void;
export const onTabMessage = <T extends keyof OnTabMessageEvents>(
  cb: OnTabMessageCallback<T>
) =>
  (
    browser.runtime.onMessage as browser.Events.Event<OnTabMessageCallback<T>>
  ).addListener(cb);
