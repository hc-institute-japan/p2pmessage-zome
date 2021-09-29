import { Orchestrator, Player } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { Base64 } from "js-base64";
import { Installables } from "../types";
import { delay } from "../utils";

const dateToTimestamp = (date: Date) => {
  const milliseconds = date.getTime();
  const microseconds = milliseconds * 1000;
  return microseconds;
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
      console.log("receiving signal...", signal);
    });
    bobby.setSignalHandler((signal) => {
      console.log("receiving signal...", signal);
    });
    carly.setSignalHandler((signal) => {
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

    // send a message and link that message to your pubkey
    const send_alice_1 = await sendMessage(messages[0])(alice_cell);
    await delay(1000);

    /*
     * SOURCE CHAIN QUERY ORDER
     */
    const send_alice_2 = await sendMessage(messages[1])(alice_cell);
    await delay(5000);

    const latest_messages_fetched_alice = await getLatestMessages(1)(
      alice_cell
    );

    console.log("nicko latest message", latest_messages_fetched_alice);
    console.log("nicko agent pubkey bobby", agent_pubkey_bobby_string);
    await delay(1000);
    const last_message_hash_string =
      latest_messages_fetched_alice[0][agent_pubkey_bobby_string][0];
    const last_message =
      latest_messages_fetched_alice[1][last_message_hash_string];

    // should be the second message since query walks the chain in reverse
    // console.log("last message", last_message[0].payload.payload);
    // NOT OK: returns the first message which is "Hello, Bobby"
    t.deepEqual(
      last_message[0].payload.payload.payload,
      "I was wondering if you were free today."
    );

    let i = 0;
    let result_array: any[] = [];
    do {
      let message_temp = await sendMessage(messages[i % 3])(alice_cell);
      await readMessage(message_temp)(bobby_cell);
      console.log("nicko at: ", i, message_temp);
      result_array.push(message_temp[0][1].payload.payload.payload);
      i = i + 1;
    } while (i < 100);
    console.log(
      "nicko consolidated results: ",
      result_array,
      result_array.length
    );
    let latest = await getLatestMessages(20)(alice_cell);
    console.log("nicko latest messages", latest);
  });

  orchestrator.run();
};

export default playground;
