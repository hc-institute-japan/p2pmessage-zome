import {
  Config,
  InstallAgentsHapps,
  TransportConfigType,
} from "@holochain/tryorama";
import path from "path";
import messaging from "./messaging";
import receipts from "./receipts";
import signals from "./signals"
import { Installables } from "./types";

// PROXY
// ct network = {
//   transport_pool: [{
//     type: TransportConfigType.Proxy,
//     sub_transport: {type: TransportConfigType.Quic},
//     proxy_config: {
//       type: ProxyConfigType.LocalProxyServer,
//       proxy_accept_config: ProxyAcceptConfig.AcceptAll
//     }
//   }],
//   bootstrap_service: "https://bootstrap.holo.host"
// };

// MEM
// const network = {
//   transport_pool: [{
//     type: TransportConfigType.Mem,
//   }]
// }


// QUIC
const network = {
  transport_pool: [
    {
      type: TransportConfigType.Quic,
    },
  ],
  bootstrap_service: "https://bootstrap-staging.holo.host/",
};


const conductorConfig = Config.gen({network});

const p2pmessagedna = path.join(__dirname, "../../p2pmessage.dna");
const installAgent: InstallAgentsHapps = [[[p2pmessagedna]]];

const installables: Installables = {
  one: installAgent,
};

messaging(conductorConfig, installables);
receipts(conductorConfig, installables);
signals(conductorConfig, installables);
//--------------------------------------------




// orchestrator.registerScenario("p2pmessage async", async (s, t) => {
//   const [alice, bobby] = await s.players([conductorConfig, conductorConfig]);
//   const [[alice_happ]] = await alice.installAgentsHapps(installation);
//   const [[bobby_happ]] = await bobby.installAgentsHapps(installation);

//   const alice_cell = alice_happ.cells[0];
//   const bobby_cell = bobby_happ.cells[0];

//   const agent_pubkey_alice = alice_happ.agent;
//   const agent_pubkey_bobby = bobby_happ.agent;

//   console.log(agent_pubkey_alice);
//   console.log(agent_pubkey_bobby);

//   const message = {
//     receiver: agent_pubkey_bobby,
//     payload: "Hello world",
//     replyTo: null,
//   };

//   const message_async_1 = {
//     receiver: agent_pubkey_bobby,
//     payload: "Read this when you get back online",
//     replyTo: null,
//   };

//   const message_async_2 = {
//     receiver: agent_pubkey_alice,
//     payload: "I have read your message",
//     replyTo: null,
//   };

//   // alice sends a message to bob
//   const send_alice = await alice_cell.call(
//     "p2pmessage",
//     "send_message",
//     message
//   );
//   await delay(2000);
//   console.log("alice sends a message to bob");
//   console.log(send_alice);
//   t.deepEqual(send_alice.author, agent_pubkey_alice);
//   t.deepEqual(send_alice.receiver, agent_pubkey_bobby);
//   t.deepEqual(send_alice.payload, "Hello world");
//   t.deepEqual(send_alice.status, { Delivered: null });

//   // bobby goes offline
//   await bobby.shutdown();
//   await delay(3000);

//   // alice sends a message to bobby but fails
//   // switches to async messaging
//   const send_async_alice = await alice_cell.call(
//     "p2pmessage",
//     "send_message",
//     message_async_1
//   );
//   for (let i = 0; i < 5; i++) {
//     console.log(i);
//     await delay(1000);
//   }
//   console.log("alice sends a message to offline bob");
//   console.log(send_async_alice);
//   t.deepEqual(send_async_alice.author, agent_pubkey_alice);
//   t.deepEqual(send_async_alice.receiver, agent_pubkey_bobby);
//   t.deepEqual(send_async_alice.payload, "Read this when you get back online");
//   t.deepEqual(send_async_alice.status, { Sent: null });

//   // alice has stored the "Sent" message to her source chain
//   const get_all_messages_alice_1 = await alice_cell.call(
//     "p2pmessage",
//     "get_all_messages",
//     null
//   );
//   await delay(1000);
//   console.log("alice gets all of her messages in her source chain");
//   console.log(get_all_messages_alice_1);
//   t.deepEqual(get_all_messages_alice_1.length, 2);
//   t.deepEqual(get_all_messages_alice_1[0].status, { Sent: null });
//   t.deepEqual(get_all_messages_alice_1[1].status, { Delivered: null });

//   // bobby comes back online
//   await bobby.startup();
//   await delay(3000);

//   // bobby gets all messages in his source chain before fetching async messages
//   const get_all_messages_bobby_1 = await bobby_cell.call(
//     "p2pmessage",
//     "get_all_messages",
//     null
//   );
//   await delay(1000);
//   console.log("bobby gets all of his messages before fetching async messages");
//   console.log(get_all_messages_bobby_1);
//   t.deepEqual(get_all_messages_bobby_1.length, 1);
//   t.deepEqual(get_all_messages_bobby_1[0].status, { Delivered: null });

//   // bobby fetches his async messages
//   const fetch_async_messages_bobby = await bobby_cell.call(
//     "p2pmessage",
//     "fetch_async_messages",
//     null
//   );
//   await delay(3000);
//   console.log("bobby fetches his async messages");
//   console.log(fetch_async_messages_bobby);
//   t.deepEqual(fetch_async_messages_bobby.length, 1);
//   t.deepEqual(fetch_async_messages_bobby[0].author, agent_pubkey_alice);
//   t.deepEqual(fetch_async_messages_bobby[0].receiver, agent_pubkey_bobby);
//   t.deepEqual(
//     fetch_async_messages_bobby[0].payload,
//     "Read this when you get back online"
//   );
//   t.deepEqual(fetch_async_messages_bobby[0].status, { Sent: null });

//   // bobby has stored the async messages into his source chain
//   const get_all_messages_bobby_2 = await bobby_cell.call(
//     "p2pmessage",
//     "get_all_messages",
//     null
//   );
//   await delay(1000);
//   console.log("bobby gets all of his messages after fetching async messages");
//   console.log(get_all_messages_bobby_2);
//   t.deepEqual(get_all_messages_bobby_2.length, 2);
//   t.deepEqual(get_all_messages_bobby_2[0].status, { Delivered: null });
//   t.deepEqual(get_all_messages_bobby_2[1].status, { Delivered: null });

//   // alice has been notified that bobby has received her message
//   const get_all_messages_alice_2 = await alice_cell.call(
//     "p2pmessage",
//     "get_all_messages",
//     null
//   );
//   await delay(1000);
//   console.log(
//     "alice gets all messages in her source chain after being notified by bobby"
//   );
//   console.log(get_all_messages_alice_2);
//   t.deepEqual(get_all_messages_alice_2.length, 3);
//   t.deepEqual(get_all_messages_alice_2[0].status, { Delivered: null });
//   t.deepEqual(get_all_messages_alice_2[1].status, { Sent: null });
//   t.deepEqual(get_all_messages_alice_2[2].status, { Delivered: null });

//   // bobby fetches his async messages after receiving all async messages
//   const fetch_async_messages_bobby_2 = await bobby_cell.call(
//     "p2pmessage",
//     "fetch_async_messages",
//     null
//   );
//   await delay(3000);
//   console.log(
//     "bobby fetches his async messages after receiving all async messages"
//   );
//   console.log(fetch_async_messages_bobby_2);
//   t.deepEqual(fetch_async_messages_bobby_2.length, 0);
// });

// orchestrator.registerScenario("p2pmessage async notify delivery", async (s, t) => {

//   const [alice, bobby, carly, diego, elise] = await s.players([
//     conductorConfig,
//     conductorConfig,
//     conductorConfig,
//     conductorConfig,
//     conductorConfig
//   ]);
//   const [[alice_happ]] = await alice.installAgentsHapps(installation);
//   const [[bobby_happ]] = await bobby.installAgentsHapps(installation);
//   const [[carly_happ]] = await carly.installAgentsHapps(installation);
//   const [[diego_happ]] = await carly.installAgentsHapps(installation);
//   const [[elise_happ]] = await carly.installAgentsHapps(installation);

//   const agent_pubkey_alice = alice_happ.agent
//   const agent_pubkey_bobby = bobby_happ.agent

//     const message_async_1 = {
//       receiver: agent_pubkey_alice,
//       payload: "Read this when you get back online",
//       replyTo: null,
//     };

//     const message_async_2 = {
//       receiver: agent_pubkey_alice,
//       payload: "I have read your message",
//       replyTo: null,
//     };

//     /* FLOW
//      * 1. ALICE GOES OFFLINE
//      * 2. BOBBY TRIES TO SEND A MESSAGE TO ALICE
//      * 3. BOBBY GOES OFFLINE
//      * 4. ALICE GOES BACK ONLINE
//      * 5. ALICE FETCHES HER ASYNC MESSAGES (ALICE RECEIVES THE BOBBY'S MESSAGE)
//      * 6. ALICE TRIES TO NOTIFY BOBBY ABOUT THE RECEIPT
//      * 7. BOBBY GOES BACK ONLINE
//      * 8. BOBBY FETCHES HIS ASYNC MESSAGES (BOBBY IS NOTIFIED ABOUT THE RECEIPT)
//      */
//     // wait for all relevant entries to be committed
//     await delay(20000);

//     // alice goes offline
//     await alice.shutdown();
//     await delay(1000);

//     // bobby sends a message to alice but fails
//     // switches to async messaging
//     const send_async_bobby = await bobby_cell.call(
//       "p2pmessage",
//       "send_message",
//       message_async_1
//     );
//     await delay(10000);
//     console.log("bobby sends a message to offline alice");
//     console.log(send_async_bobby);
//     t.deepEqual(send_async_bobby.author, agent_pubkey_bobby);
//     t.deepEqual(send_async_bobby.receiver, agent_pubkey_alice);
//     t.deepEqual(send_async_bobby.payload, "Read this when you get back online");
//     t.deepEqual(send_async_bobby.status, { Sent: null });

//     // alice comes back online
//     await alice.startup();
//     await delay(10000);

//     // bobby goes offline
//     await bobby.shutdown();
//     await delay(1000);

//     // alice fetches her async messages
//     const fetch_async_messages_alice = await alice_cell.call(
//       "p2pmessage",
//       "fetch_async_messages",
//       null
//     );
//     await delay(10000);
//     console.log("alice fetches her async messages");
//     console.log(fetch_async_messages_alice);
//     t.deepEqual(fetch_async_messages_alice.length, 1);
//     t.deepEqual(fetch_async_messages_alice[0].author, agent_pubkey_bobby);
//     t.deepEqual(fetch_async_messages_alice[0].receiver, agent_pubkey_alice);
//     t.deepEqual(
//       fetch_async_messages_alice[0].payload,
//       "Read this when you get back online"
//     );
//     t.deepEqual(fetch_async_messages_alice[0].status, { Sent: null });

//     // bobby comes back online
//     await bobby.startup();
//     await delay(10000);

//     // bobby has not been notified yet that alice received his message
//     const get_all_messages_bobby_1 = await bobby_cell.call(
//       "p2pmessage",
//       "get_all_messages",
//       null
//     );
//     await delay(1000);
//     console.log(
//       "bobby gets all of his messages before fetching async messages"
//     );
//     console.log(get_all_messages_bobby_1);
//     t.deepEqual(get_all_messages_bobby_1.length, 1);
//     t.deepEqual(get_all_messages_bobby_1[0].status, { Sent: null });

//     const get_all_messages_alice_1 = await alice_cell.call(
//       "p2pmessage",
//       "get_all_messages",
//       null
//     );
//     await delay(1000);
//     console.log("alice gets all of her messages in her source chain");
//     console.log(get_all_messages_alice_1);
//     t.deepEqual(get_all_messages_alice_1.length, 1);
//     t.deepEqual(get_all_messages_alice_1[0].status, { Delivered: null });

//     // bobby fetches his async messages to know that his message has been received
//     const fetch_async_messages_bobby_2 = await bobby_cell.call(
//       "p2pmessage",
//       "fetch_async_messages",
//       null
//     );
//     await delay(30000);
//     console.log("bobby fetches his async messages");
//     console.log(fetch_async_messages_bobby_2);
//     t.deepEqual(fetch_async_messages_bobby_2.length, 1);
//     t.deepEqual(fetch_async_messages_bobby_2[0].author, agent_pubkey_bobby);
//     t.deepEqual(fetch_async_messages_bobby_2[0].receiver, agent_pubkey_alice);
//     t.deepEqual(
//       fetch_async_messages_bobby_2[0].payload,
//       "Read this when you get back online"
//     );
//     t.deepEqual(fetch_async_messages_bobby_2[0].status, { Delivered: null });

//     // bobby has been notified that alice received his message
//     const get_all_messages_bobby_2 = await bobby_cell.call(
//       "p2pmessage",
//       "get_all_messages",
//       null
//     );
//     await delay(1000);
//     console.log("bobby gets all of his messages after fetching async messages");
//     console.log(get_all_messages_bobby_2);
//     t.deepEqual(get_all_messages_bobby_2.length, 2);
//     t.deepEqual(get_all_messages_bobby_2[0].status, { Delivered: null });
//     t.deepEqual(get_all_messages_bobby_2[1].status, { Sent: null });
//   }
// );

// });

// orchestrator.registerScenario("signal", async (s, t) => {
//   const [alice, bob] = await s.players([conductorConfig, conductorConfig]);
//   console.log("test");
//   bob. Handler((signal) => {
//     console.log("SIGNAL: ");
//     console.log(signal);
//   });

//   const [[alice_happ]] = await alice.installAgentsHapps(installation);
//   const [[bob_happ]] = await bob.installAgentsHapps(installation);

//   const alice_cell = alice_happ.cells[0];
//   const bobby_cell = bobby_happ.cells[0];

//   const agent_pubkey_alice = alice_happ.agent
//   const agent_pubkey_bobby = bobby_happ.agent

//   console.log(agent_pubkey_alice)
//   console.log(agent_pubkey_bobby)

//   const message_async_1 = {
//     receiver: agent_pubkey_alice,
//     payload: "Read this when you get back online",
//     replyTo: null
//   }

//   const message_async_2 = {
//     receiver: agent_pubkey_alice,
//     payload: "I have read your message",
//     replyTo: null
//   }

//   /* FLOW
//    * 1. ALICE GOES OFFLINE
//    * 2. BOBBY TRIES TO SEND A MESSAGE TO ALICE
//    * 3. BOBBY GOES OFFLINE
//    * 4. ALICE GOES BACK ONLINE
//    * 5. ALICE FETCHES HER ASYNC MESSAGES (ALICE RECEIVES THE BOBBY'S MESSAGE)
//    * 6. ALICE TRIES TO NOTIFY BOBBY ABOUT THE RECEIPT
//    * 7. BOBBY GOES BACK ONLINE
//    * 8. BOBBY FETCHES HIS ASYNC MESSAGES (BOBBY IS NOTIFIED ABOUT THE RECEIPT)
//    */
//   // wait for all relevant entries to be committed
//   await delay(20000);

//   // alice goes offline
//   await alice.shutdown();
//   await delay(1000);

//   // bobby sends a message to alice but fails
//   // switches to async messaging
//   const send_async_bobby = await bobby_cell.call('p2pmessage', 'send_message', message_async_1);
//   await delay(10000);
//   console.log("bobby sends a message to offline alice");
//   console.log(send_async_bobby);
//   t.deepEqual(send_async_bobby.author, agent_pubkey_bobby);
//   t.deepEqual(send_async_bobby.receiver, agent_pubkey_alice);
//   t.deepEqual(send_async_bobby.payload, "Read this when you get back online");
//   t.deepEqual(send_async_bobby.status, { Sent: null} );

//   // alice comes back online
//   await alice.startup();
//   await delay(10000);

//   // bobby goes offline
//   await bobby.shutdown();
//   await delay(1000);

//   // alice fetches her async messages
//   const fetch_async_messages_alice = await alice_cell.call('p2pmessage', 'fetch_async_messages', null);
//   await delay(10000);
//   console.log("alice fetches her async messages");
//   console.log(fetch_async_messages_alice);
//   t.deepEqual(fetch_async_messages_alice.length, 1);
//   t.deepEqual(fetch_async_messages_alice[0].author, agent_pubkey_bobby);
//   t.deepEqual(fetch_async_messages_alice[0].receiver, agent_pubkey_alice);
//   t.deepEqual(fetch_async_messages_alice[0].payload, "Read this when you get back online");
//   t.deepEqual(fetch_async_messages_alice[0].status, { Sent: null} );

//   // bobby comes back online
//   await bobby.startup();
//   await delay(10000);

//   // bobby has not been notified yet that alice received his message
//   const get_all_messages_bobby_1 = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("bobby gets all of his messages before fetching async messages");
//   console.log(get_all_messages_bobby_1);
//   t.deepEqual(get_all_messages_bobby_1.length, 1);
//   t.deepEqual(get_all_messages_bobby_1[0].status, { Sent: null} );

//   const get_all_messages_alice_1 = await alice_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("alice gets all of her messages in her source chain");
//   console.log(get_all_messages_alice_1);
//   t.deepEqual(get_all_messages_alice_1.length, 1);
//   t.deepEqual(get_all_messages_alice_1[0].status, { Delivered: null} );

//   // bobby fetches his async messages to know that his message has been received
//   const fetch_async_messages_bobby_2 = await bobby_cell.call('p2pmessage', 'fetch_async_messages', null);
//   await delay(30000);
//   console.log("bobby fetches his async messages");
//   console.log(fetch_async_messages_bobby_2);
//   t.deepEqual(fetch_async_messages_bobby_2.length, 1);
//   t.deepEqual(fetch_async_messages_bobby_2[0].author, agent_pubkey_bobby);
//   t.deepEqual(fetch_async_messages_bobby_2[0].receiver, agent_pubkey_alice);
//   t.deepEqual(fetch_async_messages_bobby_2[0].payload, "Read this when you get back online");
//   t.deepEqual(fetch_async_messages_bobby_2[0].status, { Delivered: null} );

//   // bobby has been notified that alice received his message
//   const get_all_messages_bobby_2 = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("bobby gets all of his messages after fetching async messages");
//   console.log(get_all_messages_bobby_2);
//   t.deepEqual(get_all_messages_bobby_2.length, 2);
//   t.deepEqual(get_all_messages_bobby_2[0].status, { Delivered: null} );
//   t.deepEqual(get_all_messages_bobby_2[1].status, { Sent: null} );

// });
// function get_batch_messages_on_conversation(messagerange) {
//   return (conductor, caller) =>
//     conductor.call(
//       caller,
//       "p2pmessage",
//       "get_batch_messages_on_conversation",
//       messagerange
//     );
// }

// function send_message(message) {
//   return (cell) => cell.call("p2pmessage", "send_message", message);
// }
