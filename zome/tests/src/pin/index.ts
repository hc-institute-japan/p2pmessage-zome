import { AppSignal } from "@holochain/conductor-api";
import { Orchestrator, Player } from "@holochain/tryorama";
import {
  delay,
  extractPayloadFromSignal,
  dateToTimestamp,
  serializeHash,
} from "../utils";
import { Installables } from "../types";

const handleTypeSignal = (signal: AppSignal) => () =>
  extractPayloadFromSignal(signal);

function sendMessage(message) {
  return (conductor) => conductor.call("p2pmessage", "send_message", message);
}

function pinMessage(message) {
  let timestamp = new Date();
  let message_input = {
    message_hashes: [message[0][0]],
    conversants: [message[0][1].receiver, message[0][1].author],
    status: "Pinned",
    timestamp: dateToTimestamp(timestamp),
  };
  return (conductor) =>
    conductor.call("p2pmessage", "pin_message", message_input);
}

function unpinMessage(message) {
  let timestamp = new Date();
  let message_input = {
    message_hashes: [message[0][0]],
    conversants: [message[0][1].receiver, message[0][1].author],
    status: "Unpinned",
    timestamp: dateToTimestamp(timestamp),
  };
  return (conductor) =>
    conductor.call("p2pmessage", "pin_message", message_input);
}

function getPinnedMessages(conversant) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_pinned_messages", conversant);
}

const pin = async (conductorConfig, installation: Installables) => {
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

    const message_1_result = await sendMessage(messages[0])(alice_cell);
    await delay(1000);

    const message_2_result = await sendMessage(messages[1])(alice_cell);
    await delay(1000);

    const message_3_result = await sendMessage(messages[2])(alice_cell);
    await delay(1000);
    console.log("nicko sebd", message_3_result);

    const message_4_result = await sendMessage(messages[3])(bobby_cell);
    await delay(1000);

    const message_5_result = await sendMessage(messages[4])(alice_cell);
    await delay(1000);

    const pinMessage3 = await pinMessage(message_3_result)(alice_cell);
    await delay(1000);
    console.log("nicko", pinMessage3);

    const pinnedMessages1 = await getPinnedMessages(agent_pubkey_alice)(
      bobby_cell
    );
    await delay(1000);

    t.deepEqual(
      Object.keys(pinnedMessages1[1]).includes(
        serializeHash(message_3_result[0][0])
      ),
      true
    );

    const pinMessage4 = await pinMessage(message_4_result)(bobby_cell);
    await delay(1000);

    const pinnedMessages2 = await getPinnedMessages(agent_pubkey_bobby)(
      bobby_cell
    );
    await delay(1000);

    t.deepEqual(
      Object.keys(pinnedMessages2[1]).includes(
        serializeHash(message_3_result[0][0])
      ) &&
        Object.keys(pinnedMessages2[1]).includes(
          serializeHash(message_4_result[0][0])
        ),
      true
    );

    const unpinMessage3 = await unpinMessage(message_3_result)(alice_cell);
    await delay(1000);

    const pinnedMessages3 = await getPinnedMessages(agent_pubkey_bobby)(
      alice_cell
    );
    await delay(1000);

    t.deepEqual(
      !Object.keys(pinnedMessages3[1]).includes(
        serializeHash(message_3_result[0][0])
      ) &&
        Object.keys(pinnedMessages2[1]).includes(
          serializeHash(message_4_result[0][0])
        ),
      true
    );
  });

  orchestrator.run();
};

export default pin;
