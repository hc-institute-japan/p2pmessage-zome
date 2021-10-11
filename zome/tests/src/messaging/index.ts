import { Orchestrator, Player } from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import { Base64 } from "js-base64";
import { Installables } from "../types";
import { delay, serializeHash } from "../utils";

function init() {
  return (conductor) => conductor.call("p2pmessage", "init", undefined);
}

function sendMessage(message) {
  return (conductor) => conductor.call("p2pmessage", "send_message", message);
}

function getFileBytes(file_hashes) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_file_bytes", file_hashes);
}

// HELPERS
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

const messaging = async (conductorConfig, installation: Installables) => {
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario(
    "Send and receive a message",
    async (s: ScenarioApi, t) => {
      const [alice, bobby] = await s.players([
        conductorConfig,
        conductorConfig,
      ]);

      const [[alice_happ]] = await alice.installAgentsHapps(installation.one);
      const [[bobby_happ]] = await bobby.installAgentsHapps(installation.one);

      const alice_cell = alice_happ.cells[0];
      const bobby_cell = bobby_happ.cells[0];

      const agent_pubkey_alice = alice_happ.agent;
      const agent_pubkey_bobby = bobby_happ.agent;

      let agent_pubkey_alice_string = serializeHash(agent_pubkey_alice);
      let agent_pubkey_bobby_string = serializeHash(agent_pubkey_bobby);

      // await alice.startup({});
      // await bobby.startup({});
      await delay(4000);

      let list = {};
      alice.setSignalHandler((signal) => {
        // sendMessageSignalHandler(signal, list)(agent_pubkey_alice);
        console.log("receiving signal...", signal);
      });
      bobby.setSignalHandler((signal) => {
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
      ];

      await init()(alice_cell);
      await init()(bobby_cell);
      await delay(10000);

      /*
       * alice sends 3 messages to bobby
       */
      const send_alice_1 = await sendMessage(messages[0])(alice_cell);
      await delay(1000);
      // 1-3
      t.deepEqual(send_alice_1[0][1].author, agent_pubkey_alice);
      t.deepEqual(send_alice_1[0][1].receiver, agent_pubkey_bobby);
      t.deepEqual(send_alice_1[0][1].payload.payload.payload, "Hello, Bobby");

      const send_alice_2 = await sendMessage(messages[1])(alice_cell);
      await delay(1000);
      // 4-6
      t.deepEqual(send_alice_2[0][1].author, agent_pubkey_alice);
      t.deepEqual(send_alice_2[0][1].receiver, agent_pubkey_bobby);
      t.deepEqual(
        send_alice_2[0][1].payload.payload.payload,
        "I was wondering if you were free today."
      );

      const send_alice_3 = await sendMessage(messages[2])(alice_cell);
      await delay(1000);
      // 7-9
      t.deepEqual(send_alice_3[0][1].author, agent_pubkey_alice);
      t.deepEqual(send_alice_3[0][1].receiver, agent_pubkey_bobby);
      t.deepEqual(
        send_alice_3[0][1].payload.payload.payload,
        "Would you like to go out for coffee?"
      );

      /*
       * bobby sends two message to alice
       */
      const send_bobby_1 = await sendMessage(messages[3])(bobby_cell);
      await delay(1000);
      // 10-12
      t.deepEqual(send_bobby_1[0][1].author, agent_pubkey_bobby);
      t.deepEqual(send_bobby_1[0][1].receiver, agent_pubkey_alice);
      t.deepEqual(send_bobby_1[0][1].payload.payload.payload, "Hi, Alice!");

      const send_bobby_2 = await sendMessage(messages[4])(bobby_cell);
      await delay(1000);
      // 13-15
      t.deepEqual(send_bobby_2[0][1].author, agent_pubkey_bobby);
      t.deepEqual(send_bobby_2[0][1].receiver, agent_pubkey_alice);
      t.deepEqual(
        send_bobby_2[0][1].payload.payload.payload,
        "Sure. I'd love to have coffee with you."
      );

      /*
       * alice sends one messages to bobby
       */
      const send_alice_4 = await sendMessage(messages[5])(alice_cell);
      await delay(1000);
      // 16-18
      t.deepEqual(send_alice_4[0][1].author, agent_pubkey_alice);
      t.deepEqual(send_alice_4[0][1].receiver, agent_pubkey_bobby);
      t.deepEqual(
        send_alice_4[0][1].payload.payload.payload,
        "Great! I'll see you later!"
      );
    }
  );

  orchestrator.run();
};

export default messaging;
