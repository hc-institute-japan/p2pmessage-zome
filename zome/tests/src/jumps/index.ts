import { AppSignal } from "@holochain/conductor-api";
import { Orchestrator, Player } from "@holochain/tryorama";
import { delay, extractPayloadFromSignal } from "../utils";
import { Installables } from "../types";

const handleTypeSignal = (signal: AppSignal) => () =>
  extractPayloadFromSignal(signal);

const Uint8ArrayToBase64 = (arr: Uint8Array): string => {
  return (
    "u" +
    Buffer.from(arr).toString("base64").replace(/\//g, "_").replace(/\+/g, "-")
  );
};

const dateToTimestamp = (date: Date) => {
  const milliseconds = date.getTime();
  const seconds = (milliseconds / 1000) >> 0;
  const nanoseconds = (milliseconds % 1000) * 1e6;
  const ret: [number, number] = [seconds, nanoseconds];
  return ret;
};

export const timestampToDate = (timestamp: number[]) => {
  const { 0: seconds, 1: nanoseconds } = timestamp;
  const date = new Date(seconds * 1000 + nanoseconds * 1e-6);
  return date;
};

function sendMessageWithTimestamp(message, days) {
  let time = new Date().getTime() - 1000 * 60 * 60 * 24 * days;
  message.timestamp = dateToTimestamp(new Date(time));
  return (conductor) =>
    conductor.call("p2pmessage", "send_message_with_timestamp", message);
}

function getPreviousMessages(message, conversant) {
  let input = {
    conversant: conversant,
    batch_size: 5,
    payload_type: "All",
    last_fetched_timestamp: message[0][1].timeSent,
    last_fetched_message_id: message[0][0],
  };
  return (conductor) =>
    conductor.call("p2pmessage", "get_next_batch_messages", input);
}

function getNextMessages(message, conversant) {
  let input = {
    conversant: conversant,
    batch_size: 5,
    payload_type: "All",
    last_fetched_timestamp: message[0][1].timeSent,
    last_fetched_message_id: message[0][0],
  };
  return (conductor) =>
    conductor.call("p2pmessage", "get_next_messages", input);
}

function getAdjacentMessages(message, conversant) {
  let input = {
    conversant: conversant,
    batch_size: 2,
    payload_type: "All",
    last_fetched_timestamp: message[0][1].timeSent,
    last_fetched_message_id: message[0][0],
  };
  return (conductor) =>
    conductor.call("p2pmessage", "get_adjacent_messages", input);
}

const jumps = async (conductorConfig, installation: Installables) => {
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario("Pin message test", async (s, t) => {
    const [alice, bob]: Player[] = await s.players([
      conductorConfig,
      conductorConfig,
    ]);

    const [[alice_happ]] = await alice.installAgentsHapps(installation.one);
    const [[bobby_happ]] = await bob.installAgentsHapps(installation.one);

    const alice_cell = alice_happ.cells[0];
    const bobby_cell = bobby_happ.cells[0];

    const agent_pubkey_bobby = bobby_happ.agent;
    const agent_pubkey_alice = alice_happ.agent;

    let receiptsFromSignal: any[] = [];

    alice.setSignalHandler((signal) => {
      let result = handleTypeSignal(signal)();
      let arr = Object.values(result);
      if (arr.length > 1) receiptsFromSignal.push(arr[1]);
    });

    let messages = [
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "Hello, Bobby" },
        },
        replyTo: null,
      },

      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "I was wondering if you were free today." },
        },
        replyTo: null,
      },

      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "Would you like to go out for coffee?" },
        },
        replyTo: null,
      },

      {
        receiver: agent_pubkey_alice,
        payload: {
          type: "TEXT",
          payload: { payload: "Hi, Alice!" },
        },
        replyTo: null,
      },

      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "Hi, Bobby!" },
        },
        replyTo: null,
      },
    ];

    const message_1_result = await sendMessageWithTimestamp(
      messages[0],
      4 // 4 days ago
    )(alice_cell);
    await delay(1000);

    const message_2_result = await sendMessageWithTimestamp(
      messages[1],
      2 // 2 days ago
    )(alice_cell);
    await delay(1000);

    const message_3_result = await sendMessageWithTimestamp(
      messages[2],
      0 // today
    )(alice_cell);
    await delay(1000);

    const message_4_result = await sendMessageWithTimestamp(
      messages[3],
      -2 // 2 days from now
    )(bobby_cell);
    await delay(1000);

    const message_5_result = await sendMessageWithTimestamp(
      messages[4],
      -4 // 4 days from now
    )(alice_cell);
    await delay(1000);

    console.log(
      "nicko with timestamp",
      message_5_result,
      timestampToDate(message_5_result[0][1].timeSent)
    );

    const previousMessages = await getPreviousMessages(
      message_2_result,
      agent_pubkey_bobby
    )(alice_cell);
    await delay(1000);

    const nextMessages = await getNextMessages(
      message_2_result,
      agent_pubkey_alice
    )(bobby_cell);
    await delay(1000);

    const adjacentMessages = await getAdjacentMessages(
      message_3_result,
      agent_pubkey_alice
    )(alice_cell);
    await delay(1000);

    const message_1_hash = Uint8ArrayToBase64(message_1_result[0][0]);
    const message_2_hash = Uint8ArrayToBase64(message_2_result[0][0]);
    const message_3_hash = Uint8ArrayToBase64(message_3_result[0][0]);
    const message_4_hash = Uint8ArrayToBase64(message_4_result[0][0]);
    const message_5_hash = Uint8ArrayToBase64(message_5_result[0][0]);

    const previousMessageHashes = Object.keys(previousMessages[1]);
    const nextMessageHashes = Object.keys(nextMessages[1]);
    const adjacentMessageHashes = Object.keys(adjacentMessages[1]);

    t.deepEqual(previousMessageHashes.length, 1);
    t.deepEqual(previousMessageHashes.includes(message_1_hash), true);

    t.deepEqual(nextMessageHashes.length, 3);
    t.deepEqual(
      nextMessageHashes.includes(message_3_hash) &&
        nextMessageHashes.includes(message_3_hash) &&
        nextMessageHashes.includes(message_4_hash),
      true
    );

    t.deepEqual(adjacentMessageHashes.length, 4);
    t.deepEqual(
      adjacentMessageHashes.includes(message_1_hash) &&
        adjacentMessageHashes.includes(message_2_hash) &&
        adjacentMessageHashes.includes(message_4_hash) &&
        adjacentMessageHashes.includes(message_5_hash),
      true
    );
  });

  orchestrator.run();
};

export default jumps;
