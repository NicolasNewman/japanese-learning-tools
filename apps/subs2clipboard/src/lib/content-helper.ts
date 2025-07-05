
import browser from "webextension-polyfill";

type RuntimeMessagePayload = {
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
      tabId: number;
    };
    response: null;
  };
  SEND_SUDACHI: {
    message: {
      type: "SEND_SUDACHI";
      text: string;
      id: string;
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
  };
  DEBUG_MODE_CHANGED: {
    message: {
      type: "DEBUG_MODE_CHANGED";
      enabled: boolean;
    };
    response: null;
  };
};
type OnTabMessageCallback<T extends keyof RuntimeMessagePayload> = (
  msg: RuntimeMessagePayload[T]["message"],
  sender: browser.Runtime.MessageSender,
  sendResponse: (response: RuntimeMessagePayload[T]["response"]) => void
) => void;
export const onRuntimeMessage = <T extends keyof RuntimeMessagePayload>(
  cb: OnTabMessageCallback<T>
) =>
  (
    browser.runtime.onMessage as browser.Events.Event<OnTabMessageCallback<T>>
  ).addListener(cb);


export const sendRuntimeMessage = <T extends keyof RuntimeMessagePayload>(msg: RuntimeMessagePayload[T]["message"]) => browser.runtime.sendMessage<RuntimeMessagePayload[T]["message"], void>(msg);