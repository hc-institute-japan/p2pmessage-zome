import { AppSignal } from "@holochain/conductor-api";

export const extractPayloadFromSignal = (signal: AppSignal) =>
  signal.data.payload;

export const delay = (time = 1000) =>
  new Promise((resolve) => setTimeout(resolve, time));
