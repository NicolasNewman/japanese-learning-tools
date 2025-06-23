import browser from "webextension-polyfill";

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
      tabId: number;
    };
    response: null;
  };
  UPDATE_SUDACHI: {
    message: {
      type: "UPDATE_SUDACHI";
      text: string;
      id: string;
    };
    response: null;
  }
};
export const sendMessageToTab = browser.tabs.sendMessage as <
  T extends keyof SendMessageToTabEvents
>(
  tabId: number,
  message: SendMessageToTabEvents[T]["message"]
) => Promise<SendMessageToTabEvents[T]["response"]>;