import { Orchestrator, Player } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { Base64 } from "js-base64";
import { Installables } from "../types";
import { delay } from "../utils";

const dateToTimestamp = (date: Date) => {
  const milliseconds = date.getTime();
  const seconds = (milliseconds / 1000) >> 0;
  const nanoseconds = (milliseconds % 1000) * 1e6;
  const ret: [number, number] = [seconds, nanoseconds];
  return ret;
};

function sendMessage(message) {
  return (conductor1) => {
    return conductor1.call("p2pmessage", "send_message", message);
  };
}

function readMessage(message) {
  let input = {
    message_hashes: [message[0][0]],
    sender: message[0][1].author,
    timestamp: dateToTimestamp(new Date()),
  };
  return (conductor) => conductor.call("p2pmessage", "read_message", input);
}

function getLatestMessages(batch_size) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_latest_messages", batch_size);
}

function getNextBatchMessages(batch_filter) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_next_batch_messages", batch_filter);
}

function getMessagesByAgentByTimestamp(timestamp_filter) {
  return (conductor) =>
    conductor.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter
    );
}

function sendMessageSignal(message) {
  return (conductor) => {
    return conductor.call("p2pmessage", "send_message_signal", message);
  };
}

function readMessageSignal(message) {
  let input = {
    message_hashes: [message[0][0]],
    sender: message[0][1].author,
    timestamp: dateToTimestamp(new Date()),
  };
  console.log("nicko test read input", input);
  return (conductor) =>
    conductor.call("p2pmessage", "read_message_signal", input);
}

// function sendMessageSignalHandler(signal, data) {
//   return function (sender) {
//     if (signal.data.payload.payload.GroupMessageData) {
//       const group = JSON.stringify(
//         signal.data.payload.payload.GroupMessageData.content.groupHash
//       );
//       if (!data[group]) data[group] = {};
//       const agent = JSON.stringify(sender);
//       if (data[group][agent])
//         data[group][agent].push(signal.data.payload.payload.GroupMessageData);
//       else data[group][agent] = [signal.data.payload.payload.GroupMessageData];
//     }
//   };
// }

const playground = async (conductorConfig, installation: Installables) => {
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario("p2pmessage", async (s: ScenarioApi, t) => {
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
    await delay(2000);

    let agent_pubkey_alice_string = "u" + agent_pubkey_alice.toString("base64");
    agent_pubkey_alice_string = agent_pubkey_alice_string.replace(/\//g, "_");
    agent_pubkey_alice_string = agent_pubkey_alice_string.replace(/\+/g, "-");

    let agent_pubkey_bobby_string = "u" + agent_pubkey_bobby.toString("base64");
    agent_pubkey_bobby_string = agent_pubkey_bobby_string.replace(/\//g, "_");
    agent_pubkey_bobby_string = agent_pubkey_bobby_string.replace(/\+/g, "-");

    let agent_pubkey_carly_string = "u" + agent_pubkey_carly.toString("base64");
    agent_pubkey_carly_string = agent_pubkey_carly_string.replace(/\//g, "_");
    agent_pubkey_carly_string = agent_pubkey_carly_string.replace(/\+/g, "-");

    let list = {};
    alice.setSignalHandler((signal) => {
      // sendMessageSignalHandler(signal, list)(agent_pubkey_alice);
      console.log("receiving signal...", signal);
    });
    bobby.setSignalHandler((signal) => {
      // sendMessageSignalHandler(signal, list)(agent_pubkey_bobby);
      console.log("receiving signal...", signal);
    });
    carly.setSignalHandler((signal) => {
      // sendMessageSignalHandler(signal, list)(agent_pubkey_bobby);
      console.log("receiving signal...", signal);
    });

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
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "Hi, Bobby!" },
        },
        replyTo: null,
      },
    ];

    const send_alice_1 = await sendMessageSignal(messages[0])(alice_cell);
    // await delay(5000);
    // const read_message_1 = await readMessageSignal(send_alice_1)(bobby_cell);

    // const send_alice_2 = await sendMessageSignal(messages[1])(alice_cell);
    // const read_message_2 = await readMessage(send_alice_2)(bobby_cell);

    // const send_alice_3 = await sendMessageSignal(messages[1])(alice_cell);

    // const send_alice_3 = await sendMessage(messages[2])(alice_cell, carly_cell);
    // const read_message_3 = await readMessage(send_alice_3)(bobby_cell);

    // const send_alice_4 = await sendMessage(messages[3])(bobby_cell, carly_cell);
    // const read_message_4 = await readMessage(send_alice_4)(alice_cell);

    // const send_alice_5 = await sendMessage(messages[4])(alice_cell, carly_cell);
    // const read_message_5 = await readMessage(send_alice_5)(bobby_cell);
  });

  orchestrator.run();
};

export default playground;
