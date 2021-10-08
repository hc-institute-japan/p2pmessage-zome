import { Orchestrator, Player } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { Installables } from "../types";
import {
  delay,
  timestampToDate,
  dateToTimestamp,
  serializeHash,
  transformMessage,
  sortMessagesByTimeSent,
  handleTypeSignal,
} from "../utils";

function sendMessage(message) {
  return (conductor) => conductor.call("p2pmessage", "send_message", message);
}

function getLatestMessages(batch_size) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_latest_messages", batch_size);
}

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
    conductor.call("p2pmessage", "get_previous_messages", input);
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

function getMessagesByAgentByTimestamp(timestamp_filter) {
  return (conductor) =>
    conductor.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter
    );
}

function getFileBytes(file_hashes) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_file_bytes", file_hashes);
}

function hash_file(input_text) {
  let file_hash = require("crypto")
    .createHash("sha256")
    .update(input_text)
    .digest("hex");
  return file_hash;
}

// HELPERS

// function evaluateMessagesFromSignal(messagesFromSignal, messages, t) {
//   Object.keys(messagesFromSignal).forEach((group) => {
//     Object.keys(messagesFromSignal[group]).forEach((agent) => {
//       t.deepEqual(
//         messagesFromSignal[group][agent].filter(
//           (v, i, a) =>
//             a.findIndex((t) => JSON.stringify(t) === JSON.stringify(v)) === i
//         ),
//         messages.filter(
//           (message) =>
//             JSON.stringify(message.content.sender) !== agent &&
//             JSON.stringify(message.content.groupHash) === group
//         )
//       );
//     });
//   });
// }

const getters = async (conductorConfig, installation: Installables) => {
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario("fetch messages", async (s: ScenarioApi, t) => {
    const [alice, bobby, carly] = await s.players([
      conductorConfig,
      conductorConfig,
      conductorConfig,
    ]);

    const [[alice_happ]] = await alice.installAgentsHapps(installation.one);
    const [[bobby_happ]] = await bobby.installAgentsHapps(installation.one);
    const [[carly_happ]] = await carly.installAgentsHapps(installation.one);

    const alice_cell = alice_happ.cells[0];
    const bobby_cell = bobby_happ.cells[0];
    const carly_cell = carly_happ.cells[0];

    const agent_pubkey_alice = alice_happ.agent;
    const agent_pubkey_bobby = bobby_happ.agent;
    const agent_pubkey_carly = carly_happ.agent;

    let agent_pubkey_alice_string = serializeHash(agent_pubkey_alice);
    let agent_pubkey_bobby_string = serializeHash(agent_pubkey_bobby);
    let agent_pubkey_carly_string = serializeHash(agent_pubkey_carly);

    // let list = {};
    // alice.setSignalHandler((signal) => {
    //   // sendMessageSignalHandler(signal, list)(agent_pubkey_alice);
    //   console.log("receiving signal...", signal);
    // });
    // bobby.setSignalHandler((signal) => {
    //   // sendMessageSignalHandler(signal, list)(agent_pubkey_bobby);
    //   console.log("receiving signal...", signal);
    // });

    const messages = [
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
        receiver: agent_pubkey_alice,
        payload: {
          type: "TEXT",
          payload: {
            payload: "Sure. I'd love to have coffee with you.",
          },
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "Great! I'll see you later!" },
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "Hi, Bobby. This is Carly." },
        },
        replyTo: null,
      },
    ];

    /*
     * alice sends 3 messages to bobby
     */
    const send_alice_1 = await sendMessage(messages[0])(alice_cell);
    await delay(1000);

    const send_alice_2 = await sendMessage(messages[1])(alice_cell);
    await delay(1000);

    const send_alice_3 = await sendMessage(messages[2])(alice_cell);
    await delay(1000);

    /*
     * bobby sends two message to alice
     */
    const send_bobby_1 = await sendMessage(messages[3])(bobby_cell);
    await delay(1000);

    const send_bobby_2 = await sendMessage(messages[4])(bobby_cell);
    await delay(1000);

    /*
     * alice sends one messages to bobby
     */
    const send_alice_4 = await sendMessage(messages[5])(alice_cell);
    await delay(1000);

    /*
     * carly sends one messages to bobby
     */
    const send_carly_1 = await sendMessage(messages[6])(carly_cell);
    await delay(1000);

    // deepEqual(actual, expected)

    /*
     * alice gets her latest messages (batch size 3)
     */
    const alice_latest_messages_1 = await getLatestMessages(3)(alice_cell);
    await delay(1000);

    let message_contents = alice_latest_messages_1[1];
    let alice_latest_messages: any[] = [];
    for (let message_hash in message_contents) {
      let message_and_receipt = message_contents[message_hash];
      let [message, receipt_array] = message_and_receipt;
      alice_latest_messages.push(message);
    }
    sortMessagesByTimeSent(alice_latest_messages);
    let transformed_alice_latest = alice_latest_messages.map((message) =>
      transformMessage(message)
    );
    // 1
    t.deepEqual(transformed_alice_latest, messages.slice(3, 6).reverse());

    /*
     * bobby gets his latest messages (batch size 5)
     */
    const bobby_latest_messages_1 = await getLatestMessages(5)(bobby_cell);
    await delay(1000);

    message_contents = bobby_latest_messages_1[1];
    let bobby_latest_messages: any[] = [];
    for (let message_hash in message_contents) {
      let message_and_receipt = message_contents[message_hash];
      let [message, receipt_array] = message_and_receipt;
      bobby_latest_messages.push(message);
    }
    sortMessagesByTimeSent(bobby_latest_messages);
    let transformed_bobby_latest = bobby_latest_messages.map((message) =>
      transformMessage(message)
    );
    // 2
    t.deepEqual(transformed_bobby_latest, messages.slice(1, 7).reverse());
  });

  orchestrator.run();

  orchestrator = new Orchestrator();

  orchestrator.registerScenario("Jump to date", async (s, t) => {
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

    const message_1_hash = serializeHash(message_1_result[0][0]);
    const message_2_hash = serializeHash(message_2_result[0][0]);
    const message_3_hash = serializeHash(message_3_result[0][0]);
    const message_4_hash = serializeHash(message_4_result[0][0]);
    const message_5_hash = serializeHash(message_5_result[0][0]);

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

export default getters;
