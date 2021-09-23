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

function getMessagesLinks(agent) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_messages_links", agent);
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

    // send a message and link that message to your pubkey
    const send_alice_1 = await sendMessage(messages[0])(alice_cell);
    await delay(10000);

    /*
     * LINK 1: sender agent pubkey -link-> message
     */
    // alice gets messages linked to herself
    const links_1_fetched_by_alice = await getMessagesLinks({
      base: agent_pubkey_alice,
      tag: "messages_1",
    })(alice_cell);
    await delay(3000);

    // bobby gets messages linked to alice
    const links_1_fetched_by_bobby = await getMessagesLinks({
      base: agent_pubkey_alice,
      tag: "messages_1",
    })(bobby_cell);
    await delay(3000);

    console.log("message links 1 fetched by alice: ", links_1_fetched_by_alice);
    console.log("message links 1 fetched by bobby: ", links_1_fetched_by_bobby);

    //         |      CREATOR     |
    //         |  self  |  other  |
    // --------+--------+---------|
    // B self  |   OK   |   OK    |
    // A ------+--------+---------|
    // S other |   OK   |  NOT OK |
    // E ------+--------+---------|

    // cannot fetch link to profiles created by other agents based on them

    // OK: fetching a link created by yourself with yourself as base and a message as target
    t.deepEqual(links_1_fetched_by_alice.length, 1);
    // NOT OK: fetching a link created by another agent with that agent as base and a message as target
    t.deepEqual(links_1_fetched_by_bobby.length, 1);

    /*
     * LINK 2: receiver agent pubkey -link-> message
     */
    const links_2_fetched_by_alice = await getMessagesLinks({
      base: agent_pubkey_bobby,
      tag: "messages_2",
    })(alice_cell);
    await delay(3000);

    const links_2_fetched_by_bobby = await getMessagesLinks({
      base: agent_pubkey_bobby,
      tag: "messages_2",
    })(bobby_cell);
    await delay(3000);

    console.log("message links 2 fetched by alice: ", links_2_fetched_by_alice);
    console.log("message links 2 fetched by bobby: ", links_2_fetched_by_bobby);

    // OK: fetching a link created by yourself with another agent as base and a message as target
    t.deepEqual(links_2_fetched_by_alice.length, 1);
    // OK: fetching a link created by another agent with yourself as the base and a message as target
    t.deepEqual(links_2_fetched_by_bobby.length, 1);

    /*
     * SOURCE CHAIN QUERY ORDER
     */
    // const send_alice_2 = await sendMessage(messages[1])(alice_cell);
    // await delay(5000);

    // const latest_messages_fetched_alice = await getLatestMessages(1)(
    //   alice_cell
    // );
    // await delay(1000);
    // const last_message_hash_string =
    //   latest_messages_fetched_alice[0][agent_pubkey_bobby_string][0];
    // const last_message =
    //   latest_messages_fetched_alice[1][last_message_hash_string];

    // // should be the second message since query walks the chain in reverse
    // // console.log("last message", last_message[0].payload.payload);
    // // NOT OK: returns the first message which is "Hello, Bobby"
    // t.deepEqual(
    //   last_message[0].payload.payload.payload,
    //   "I was wondering if you were free today."
    // );

    // let i = 0;
    // let result_array: any[] = [];
    // do {
    //   let message_temp = await sendMessage(messages[i % 3])(alice_cell);
    //   // await readMessage(message_temp);
    //   console.log("nicko at: ", i, message_temp);
    //   result_array.push(message_temp[0][1].payload.payload.payload);
    //   i = i + 1;
    // } while (i < 60);
    // console.log(
    //   "nicko consolidated results: ",
    //   result_array,
    //   result_array.length
    // );
  });

  orchestrator.run();
};

export default playground;
