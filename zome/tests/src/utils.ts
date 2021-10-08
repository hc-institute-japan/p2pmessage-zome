import { AppSignal } from "@holochain/conductor-api";
import { Base64 } from "js-base64";

export const delay = (time = 1000) =>
  new Promise((resolve) => setTimeout(resolve, time));

export const dateToTimestamp = (date: Date) => {
  const milliseconds = date.getTime();
  const microseconds = milliseconds * 1000;
  return microseconds;
};

export const timestampToDate = (timestamp: number) => {
  const milliseconds = timestamp / 1000;
  const date = new Date(milliseconds);
  return date;
};

export const serializeHash = (hash) => {
  return `u${Base64.fromUint8Array(hash, true)}`;
};

export const transformMessage = (returnedMessage: any) => {
  let message = {
    receiver: returnedMessage.receiver,
    payload: {
      type: returnedMessage.payload.type,
      payload: returnedMessage.payload.payload,
    },
    replyTo: returnedMessage.replyTo,
  };
  return message;
};

export const sortMessagesByTimeSent = (messageArray) => {
  messageArray.sort((x, y) => {
    let timestampX = x.timeSent.valueOf();
    let timestampY = y.timeSent.valueOf();
    return timestampX < timestampY ? 1 : -1;
  });
};

export const hash_file = (input_text) => {
  let file_hash = require("crypto")
    .createHash("sha256")
    .update(input_text)
    .digest("hex");
  return file_hash;
};

export const handleTypeSignal = (signal: AppSignal) => () =>
  extractPayloadFromSignal(signal);

export const extractPayloadFromSignal = (signal: AppSignal) =>
  signal.data.payload;
