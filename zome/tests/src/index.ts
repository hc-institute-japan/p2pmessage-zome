import { Orchestrator, Config, InstallAgentsHapps, Player } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from '@holochain/tryorama'
import path from 'path'
import { Base64 } from "js-base64";

// PROXY
// const network = {
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

// QUIC
const network = {
  transport_pool: [{
    type: TransportConfigType.Quic,
  }],
  bootstrap_service: "https://bootstrap.holo.host"
}

// MEM
// const network = {
//   transport_pool: [{
//     type: TransportConfigType.Mem,
//   }]
// }

const conductorConfig = Config.gen({network});

const p2pmessagedna = path.join(__dirname, '../../p2pmessage.dna.gz')
const installation: InstallAgentsHapps = [
  [[p2pmessagedna]]
]

const orchestrator = new Orchestrator()

const delay = (ms) => new Promise((r) => setTimeout(r, ms));

function get_batch_messages_on_conversation(messagerange) {
  return (conductor, caller) => 
  conductor.call(caller, "p2pmessage", "get_batch_messages_on_conversation", messagerange);
}

function send_message(message) {
  return (cell) => 
    cell.call('p2pmessage', 'send_message', message)
}

export function serializeHash(hash) {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

// orchestrator.registerScenario("p2pmessage", async (s, t) => {

//   const [player]: Player[] = await s.players([conductorConfig]);
//   const aliceKey = await player.adminWs().generateAgentPubKey();
//   const dnas = [
//     {
//       path: p2pmessagedna,
//       nick: `my_cell_nick`,
//       properties: { progenitors: [serializeHash(aliceKey)] },
//       membrane_proof: undefined,
//     },
//   ];

//   const alice_happ = await player._installHapp({
//     installed_app_id: `my_app:12345`, // my_app with some unique installed id value
//     agent_key: aliceKey,
//     dnas,
//   });
//   const bobby_happ = await player._installHapp({
//     installed_app_id: `my_app:1234`, // my_app with some unique installed id value
//     agent_key: await player.adminWs().generateAgentPubKey(),
//     dnas,
//   });
//   const carly_happ = await player._installHapp({
//     installed_app_id: `my_app:123`, // my_app with some unique installed id value
//     agent_key: await player.adminWs().generateAgentPubKey(),
//     dnas,
//   });

//   const alice_cell = alice_happ.cells[0];
//   const bobby_cell = bobby_happ.cells[0];
//   const carly_cell = carly_happ.cells[0];

//   const agent_pubkey_alice = alice_happ.agent
//   const agent_pubkey_bobby = bobby_happ.agent
//   const agent_pubkey_carly = carly_happ.agent

//   console.log(agent_pubkey_alice)
//   console.log(agent_pubkey_bobby)
//   console.log(agent_pubkey_carly)

//   const message = {
//       receiver: agent_pubkey_bobby,
//       payload: "Hello world",
//       reply_to: null
//   };
  
//   const message_2 = {
//       receiver: agent_pubkey_bobby,
//       payload: "Hello world 2",
//       reply_to: null
//   }

//   const message_3 = {
//       receiver: agent_pubkey_alice,
//       payload: "Hello alice",
//       reply_to: null
//   }

//   const message_4 = {
//       receiver: agent_pubkey_alice,
//       payload: "I am Carly",
//       reply_to: null
//   }

//   const message_late_1 = {
//       receiver: agent_pubkey_alice,
//       payload: "Hello again",
//       reply_to: null
//   }

//   const message_late_2 = {
//       receiver: agent_pubkey_alice,
//       payload: "Am I bothering you",
//       reply_to: null
//   }

//   const message_async = {
//     receiver: agent_pubkey_bobby,
//     payload: "Read this when you get back online",
//     reply_to: null
//   }
  
//   // alice sends a message to bob
//   const send_alice = await alice_cell.call('p2pmessage', 'send_message', message);
//   await delay(5000);
//   console.log("alice sends a message to bob");
//   console.log(send_alice);
//   t.deepEqual(send_alice.author, agent_pubkey_alice);
//   t.deepEqual(send_alice.receiver, agent_pubkey_bobby);
//   t.deepEqual(send_alice.payload, "Hello world");
//   t.deepEqual(send_alice.status, { Delivered: null});

//   // alice sends another message to bob
//   const send_alice_2 = await alice_cell.call('p2pmessage', 'send_message', message_2)
//   await delay(5000);
//   console.log("alice sends a message to bob");
//   console.log(send_alice_2);
//   t.deepEqual(send_alice_2.author, agent_pubkey_alice);
//   t.deepEqual(send_alice_2.receiver, agent_pubkey_bobby);
//   t.deepEqual(send_alice_2.payload, "Hello world 2");
//   t.deepEqual(send_alice_2.status, { Delivered: null});

//   // bob replies to a message of alice
//   const replied_message = send_alice_2;
//   const reply = {
//     receiver: agent_pubkey_alice,
//     payload: "Hello back",
//     reply_to: replied_message
//   }
  
//   const reply_bobby = await bobby_cell.call('p2pmessage', 'send_message', reply);
//   await delay(5000);
//   console.log("bob replies to a message of alice");
//   console.log(reply_bobby);
//   t.deepEqual(reply_bobby.author, agent_pubkey_bobby);
//   t.deepEqual(reply_bobby.receiver, agent_pubkey_alice);
//   t.deepEqual(reply_bobby.payload, "Hello back");
//   t.deepEqual(reply_bobby.status, { Delivered: null});

//   // alice gets all messages in her source chain
//   const all_messages_alice = await alice_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(10000);
//   console.log("alice gets all messages in her source chain");
//   console.log(all_messages_alice);
//   t.deepEqual(all_messages_alice.length, 3);

//   // bob gets all messages in his source chain
//   const all_messages_bobby = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("bob gets all messages in his source chain");
//   console.log(all_messages_bobby);
//   t.deepEqual(all_messages_bobby.length, 3);

//   // carly sends a message to alice
//   const send_carly = await carly_cell.call('p2pmessage', 'send_message', message_3);
//   await delay(1000);
//   console.log("carly sends a message to alice");
//   console.log(send_carly);
//   t.deepEqual(send_carly.author, agent_pubkey_carly);
//   t.deepEqual(send_carly.receiver, agent_pubkey_alice);
//   t.deepEqual(send_carly.payload, "Hello alice");
//   t.deepEqual(send_carly.status, { Delivered: null});

//   // carly sends another message to alice
//   const send_carly_2 = await carly_cell.call('p2pmessage', 'send_message', message_4);
//   await delay(1000);
//   console.log("carly sends message to alice again");
//   console.log(send_carly_2);
//   t.deepEqual(send_carly_2.author, agent_pubkey_carly);
//   t.deepEqual(send_carly_2.receiver, agent_pubkey_alice);
//   t.deepEqual(send_carly_2.payload, "I am Carly");
//   t.deepEqual(send_carly_2.status, { Delivered: null});

//   // alice has messages from bobby and carly in her source chain
//   const messages_in_alice_from_both = await alice_cell.call('p2pmessage', 'get_all_messages_from_addresses', [agent_pubkey_bobby, agent_pubkey_carly])
//   await delay(1000);
//   console.log("alice gets her messages from bobby and carly");
//   console.log(messages_in_alice_from_both);
//   t.deepEqual(messages_in_alice_from_both.length, 2);
//   t.deepEqual(messages_in_alice_from_both[0].messages.length + messages_in_alice_from_both[1].messages.length, 3);

//   // author order may be arbitrary so the following assertions can fail
//   // t.deepEqual(messages_in_alice_from_both[0].messages.length, 2);
//   // t.deepEqual(messages_in_alice_from_both[1].messages.length, 1);

//   // message order is still arbitrary
//   // t.deepEqual(messages_in_alice_from_both[0].messages[0].payload, "Hello back");
//   // t.deepEqual(messages_in_alice_from_both[1].messages[0].payload, "Hello alice");
//   // t.deepEqual(messages_in_alice_from_both[1].messages[1].payload, "I am Carly");

//   const send_carly_3 = await carly_cell.call('p2pmessage', 'send_message', message_late_1);
//   await delay(1000);
//   console.log("carly sends message to alice again");
//   console.log(send_carly_3);
//   t.deepEqual(send_carly_3.author, agent_pubkey_carly);
//   t.deepEqual(send_carly_3.receiver, agent_pubkey_alice);
//   t.deepEqual(send_carly_3.payload, "Hello again");
//   t.deepEqual(send_carly_3.status, { Delivered: null});

//   await delay(10000);

//   const send_carly_4 = await carly_cell.call('p2pmessage', 'send_message', message_late_2);
//   await delay(1000);
//   console.log("carly sends message to alice again");
//   console.log(send_carly_4);
//   t.deepEqual(send_carly_4.author, agent_pubkey_carly);
//   t.deepEqual(send_carly_4.receiver, agent_pubkey_alice);
//   t.deepEqual(send_carly_4.payload, "Am I bothering you");
//   t.deepEqual(send_carly_4.status, { Delivered: null});

//   const last_message = {
//       author: agent_pubkey_carly,
//       last_message_timestamp_seconds: send_carly_4.time_sent[0]+2
//   };

//   const batch_messages = await alice_cell.call('p2pmessage', 'get_batch_messages_on_conversation', last_message);
//   await delay(1000);
//   console.log("alice batch fetches her messages");
//   console.log(batch_messages);
//   t.deepEqual(batch_messages.length, 1);

// });

// orchestrator.registerScenario("p2pmessage async", async (s, t) => {
    
//   const [alice, bobby] = await s.players([conductorConfig, conductorConfig]);
//   const [[alice_happ]] = await alice.installAgentsHapps(installation);
//   const [[bobby_happ]] = await bobby.installAgentsHapps(installation);

//   const alice_cell = alice_happ.cells[0];
//   const bobby_cell = bobby_happ.cells[0];

//   const agent_pubkey_alice = alice_happ.agent
//   const agent_pubkey_bobby = bobby_happ.agent

//   console.log(agent_pubkey_alice)
//   console.log(agent_pubkey_bobby)

//   const message = {
//       receiver: agent_pubkey_bobby,
//       payload: "Hello world",
//       reply_to: null
//   };

//   const message_async_1 = {
//     receiver: agent_pubkey_bobby,
//     payload: "Read this when you get back online",
//     reply_to: null
//   }

//   const message_async_2 = {
//     receiver: agent_pubkey_alice,
//     payload: "I have read your message",
//     reply_to: null
//   }
  
//   // alice sends a message to bob
//   const send_alice = await alice_cell.call('p2pmessage', 'send_message', message);
//   await delay(5000);
//   console.log("alice sends a message to bob");
//   console.log(send_alice);
//   t.deepEqual(send_alice.author, agent_pubkey_alice);
//   t.deepEqual(send_alice.receiver, agent_pubkey_bobby);
//   t.deepEqual(send_alice.payload, "Hello world");
//   t.deepEqual(send_alice.status, { Delivered: null});

//   // bobby goes offline
//   await bobby.shutdown();
//   await delay(3000);

//   // alice sends a message to bobby but fails
//   // switches to async messaging
//   const send_async_alice = await alice_cell.call('p2pmessage', 'send_message', message_async_1)
//   for (let i=0; i<5; i++) {console.log(i);await delay(1000)};
//   console.log("alice sends a message to offline bob");
//   console.log(send_async_alice);
//   t.deepEqual(send_async_alice.author, agent_pubkey_alice);
//   t.deepEqual(send_async_alice.receiver, agent_pubkey_bobby);
//   t.deepEqual(send_async_alice.payload, "Read this when you get back online");
//   t.deepEqual(send_async_alice.status, { Sent: null} );

//   // alice has stored the "Sent" message to her source chain
//   const get_all_messages_alice_1 = await alice_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("alice gets all of her messages in her source chain");
//   console.log(get_all_messages_alice_1);
//   t.deepEqual(get_all_messages_alice_1.length, 2);
//   t.deepEqual(get_all_messages_alice_1[0].status, { Sent: null} );
//   t.deepEqual(get_all_messages_alice_1[1].status, { Delivered: null} );
  
//   // bobby comes back online
//   await bobby.startup();
//   await delay(3000);

//   // bobby gets all messages in his source chain before fetching async messages
//   const get_all_messages_bobby_1 = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("bobby gets all of his messages before fetching async messages");
//   console.log(get_all_messages_bobby_1);
//   t.deepEqual(get_all_messages_bobby_1.length, 1);
//   t.deepEqual(get_all_messages_bobby_1[0].status, { Delivered: null} );

//   // bobby fetches his async messages
//   const fetch_async_messages_bobby = await bobby_cell.call('p2pmessage', 'fetch_async_messages', null);
//   await delay(3000);
//   console.log("bobby fetches his async messages");
//   console.log(fetch_async_messages_bobby);
//   t.deepEqual(fetch_async_messages_bobby.length, 1);
//   t.deepEqual(fetch_async_messages_bobby[0].author, agent_pubkey_alice);
//   t.deepEqual(fetch_async_messages_bobby[0].receiver, agent_pubkey_bobby);
//   t.deepEqual(fetch_async_messages_bobby[0].payload, "Read this when you get back online");
//   t.deepEqual(fetch_async_messages_bobby[0].status, { Sent: null} );

//   // bobby has stored the async messages into his source chain
//   const get_all_messages_bobby_2 = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("bobby gets all of his messages after fetching async messages");
//   console.log(get_all_messages_bobby_2);
//   t.deepEqual(get_all_messages_bobby_2.length, 2);
//   t.deepEqual(get_all_messages_bobby_2[0].status, { Delivered: null} );
//   t.deepEqual(get_all_messages_bobby_2[1].status, { Delivered: null} );

//   // alice has been notified that bobby has received her message
//   const get_all_messages_alice_2 = await alice_cell.call('p2pmessage', 'get_all_messages', null);
//   await delay(1000);
//   console.log("alice gets all messages in her source chain after being notified by bobby");
//   console.log(get_all_messages_alice_2);
//   t.deepEqual(get_all_messages_alice_2.length, 3);
//   t.deepEqual(get_all_messages_alice_2[0].status, { Delivered: null} );
//   t.deepEqual(get_all_messages_alice_2[1].status, { Sent: null} );
//   t.deepEqual(get_all_messages_alice_2[2].status, { Delivered: null} );

//   // bobby fetches his async messages after receiving all async messages
//   const fetch_async_messages_bobby_2 = await bobby_cell.call('p2pmessage', 'fetch_async_messages', null);
//   await delay(3000);
//   console.log("bobby fetches his async messages after receiving all async messages");
//   console.log(fetch_async_messages_bobby_2);
//   t.deepEqual(fetch_async_messages_bobby_2.length, 0);

// });

orchestrator.registerScenario("p2pmessage async notify delivery", async (s, t) => {
    
  const [alice, bobby, carly, diego, elise] = await s.players([
    conductorConfig,
    conductorConfig,
    conductorConfig,
    conductorConfig, 
    conductorConfig
  ]);
  const [[alice_happ]] = await alice.installAgentsHapps(installation);
  const [[bobby_happ]] = await bobby.installAgentsHapps(installation);
  const [[carly_happ]] = await carly.installAgentsHapps(installation);
  const [[diego_happ]] = await carly.installAgentsHapps(installation);
  const [[elise_happ]] = await carly.installAgentsHapps(installation);


  const alice_cell = alice_happ.cells[0];
  const bobby_cell = bobby_happ.cells[0];

  const agent_pubkey_alice = alice_happ.agent
  const agent_pubkey_bobby = bobby_happ.agent

  console.log(agent_pubkey_alice)
  console.log(agent_pubkey_bobby)

  const message_async_1 = {
    receiver: agent_pubkey_alice,
    payload: "Read this when you get back online",
    reply_to: null
  }

  const message_async_2 = {
    receiver: agent_pubkey_alice,
    payload: "I have read your message",
    reply_to: null
  }
  
  /* FLOW
   * 1. ALICE GOES OFFLINE
   * 2. BOBBY TRIES TO SEND A MESSAGE TO ALICE
   * 3. BOBBY GOES OFFLINE
   * 4. ALICE GOES BACK ONLINE
   * 5. ALICE FETCHES HER ASYNC MESSAGES (ALICE RECEIVES THE BOBBY'S MESSAGE)
   * 6. ALICE TRIES TO NOTIFY BOBBY ABOUT THE RECEIPT
   * 7. BOBBY GOES BACK ONLINE
   * 8. BOBBY FETCHES HIS ASYNC MESSAGES (BOBBY IS NOTIFIED ABOUT THE RECEIPT)
   */
  // wait for all relevant entries to be committed
  await delay(20000);

  // alice goes offline
  await alice.shutdown();
  await delay(1000);

  // bobby sends a message to alice but fails
  // switches to async messaging
  const send_async_bobby = await bobby_cell.call('p2pmessage', 'send_message', message_async_1);
  await delay(10000);
  console.log("bobby sends a message to offline alice");
  console.log(send_async_bobby);
  t.deepEqual(send_async_bobby.author, agent_pubkey_bobby);
  t.deepEqual(send_async_bobby.receiver, agent_pubkey_alice);
  t.deepEqual(send_async_bobby.payload, "Read this when you get back online");
  t.deepEqual(send_async_bobby.status, { Sent: null} );

  // alice comes back online
  await alice.startup();
  await delay(10000);

  // bobby goes offline
  await bobby.shutdown();
  await delay(1000);

  // alice fetches her async messages
  const fetch_async_messages_alice = await alice_cell.call('p2pmessage', 'fetch_async_messages', null);
  await delay(10000);
  console.log("alice fetches her async messages");
  console.log(fetch_async_messages_alice);
  t.deepEqual(fetch_async_messages_alice.length, 1);
  t.deepEqual(fetch_async_messages_alice[0].author, agent_pubkey_bobby);
  t.deepEqual(fetch_async_messages_alice[0].receiver, agent_pubkey_alice);
  t.deepEqual(fetch_async_messages_alice[0].payload, "Read this when you get back online");
  t.deepEqual(fetch_async_messages_alice[0].status, { Sent: null} );

  // bobby comes back online
  await bobby.startup();
  await delay(10000);

  // bobby has not been notified yet that alice received his message
  const get_all_messages_bobby_1 = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
  await delay(1000);
  console.log("bobby gets all of his messages before fetching async messages");
  console.log(get_all_messages_bobby_1);
  t.deepEqual(get_all_messages_bobby_1.length, 1);
  t.deepEqual(get_all_messages_bobby_1[0].status, { Sent: null} );

  const get_all_messages_alice_1 = await alice_cell.call('p2pmessage', 'get_all_messages', null);
  await delay(1000);
  console.log("alice gets all of her messages in her source chain");
  console.log(get_all_messages_alice_1);
  t.deepEqual(get_all_messages_alice_1.length, 1);
  t.deepEqual(get_all_messages_alice_1[0].status, { Delivered: null} );

  // bobby fetches his async messages to know that his message has been received
  const fetch_async_messages_bobby_2 = await bobby_cell.call('p2pmessage', 'fetch_async_messages', null);
  await delay(30000);
  console.log("bobby fetches his async messages");
  console.log(fetch_async_messages_bobby_2);
  t.deepEqual(fetch_async_messages_bobby_2.length, 1);
  t.deepEqual(fetch_async_messages_bobby_2[0].author, agent_pubkey_bobby);
  t.deepEqual(fetch_async_messages_bobby_2[0].receiver, agent_pubkey_alice);
  t.deepEqual(fetch_async_messages_bobby_2[0].payload, "Read this when you get back online");
  t.deepEqual(fetch_async_messages_bobby_2[0].status, { Delivered: null} );

  // bobby has been notified that alice received his message
  const get_all_messages_bobby_2 = await bobby_cell.call('p2pmessage', 'get_all_messages', null);
  await delay(1000);
  console.log("bobby gets all of his messages after fetching async messages");
  console.log(get_all_messages_bobby_2);
  t.deepEqual(get_all_messages_bobby_2.length, 2);
  t.deepEqual(get_all_messages_bobby_2[0].status, { Delivered: null} );
  t.deepEqual(get_all_messages_bobby_2[1].status, { Sent: null} );

});

orchestrator.run();

