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

orchestrator.registerScenario("p2pmessage", async (s, t) => {

  const [player]: Player[] = await s.players([conductorConfig]);
  const aliceKey = await player.adminWs().generateAgentPubKey();
  const dnas = [
    {
      path: p2pmessagedna,
      nick: `my_cell_nick`,
      properties: { progenitors: [serializeHash(aliceKey)] },
      membrane_proof: undefined,
    },
  ];

  const alice_happ = await player._installHapp({
    installed_app_id: `my_app:12345`, // my_app with some unique installed id value
    agent_key: aliceKey,
    dnas,
  });
  const bobby_happ = await player._installHapp({
    installed_app_id: `my_app:1234`, // my_app with some unique installed id value
    agent_key: await player.adminWs().generateAgentPubKey(),
    dnas,
  });
  const carly_happ = await player._installHapp({
    installed_app_id: `my_app:123`, // my_app with some unique installed id value
    agent_key: await player.adminWs().generateAgentPubKey(),
    dnas,
  });

  const alice_cell = alice_happ.cells[0];
  const bobby_cell = bobby_happ.cells[0];
  const carly_cell = carly_happ.cells[0];

  const agent_pubkey_alice = alice_happ.agent;
  const agent_pubkey_bobby = bobby_happ.agent;
  const agent_pubkey_carly = carly_happ.agent;

  var agent_pubkey_alice_string = "u" + agent_pubkey_alice.toString('base64')
  agent_pubkey_alice_string = agent_pubkey_alice_string.replace(/\//g, "_");
  agent_pubkey_alice_string = agent_pubkey_alice_string.replace(/\+/g, "-");

  var agent_pubkey_bobby_string = "u" + agent_pubkey_bobby.toString('base64')
  agent_pubkey_bobby_string = agent_pubkey_bobby_string.replace(/\//g, "_");
  agent_pubkey_bobby_string = agent_pubkey_bobby_string.replace(/\+/g, "-");

  var agent_pubkey_carly_string = "u" + agent_pubkey_carly.toString('base64')
  agent_pubkey_carly_string = agent_pubkey_carly_string.replace(/\//g, "_");
  agent_pubkey_carly_string = agent_pubkey_carly_string.replace(/\+/g, "-");

  console.log(agent_pubkey_alice)
  console.log(agent_pubkey_bobby)
  console.log(agent_pubkey_carly)

  const message_1 = {
    receiver: agent_pubkey_bobby,
    payload: "Hello, Bobby.",
    reply_to: null
  };
  
  const message_2 = {
    receiver: agent_pubkey_bobby,
    payload: "I was wondering if you were free today.",
    reply_to: null
  }

  const message_3 = {
    receiver: agent_pubkey_bobby,
    payload: "Would you like to go out for coffee?",
    reply_to: null
  }

  const message_4 = {
    receiver: agent_pubkey_alice,
    payload: "Hi, Alice!",
    reply_to: null
  }

  // reply in this sequence

  const message_6 = {
    receiver: agent_pubkey_bobby,
    payload: "Hi, Bobby!",
    reply_to: null
  }

  const message_7 = {
    receiver: agent_pubkey_bobby,
    payload: "I have an extra ticket to a movie later. Would you want to come with me?",
    reply_to: null
  }

  const message_8 = {
    receiver: agent_pubkey_carly,
    payload: "Hey, Carly. I'm sorry but I already made plans. maybe some other time?",
    reply_to: null
  }

  const message_9 = {
    receiver: agent_pubkey_bobby,
    payload: "Great! I'll see you later!",
    reply_to: null
  }

  // alice sends 3 messages to bobby
  console.log("Alice sends three messages to Bobby");
  const send_alice_1 = await alice_cell.call('p2pmessage', 'send_message', message_1);
  await delay(1000);
  console.log(send_alice_1);
  t.deepEqual(send_alice_1[0].author, agent_pubkey_alice);
  t.deepEqual(send_alice_1[0].receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice_1[0].payload, "Hello, Bobby.");

  const send_alice_2 = await alice_cell.call('p2pmessage', 'send_message', message_2);
  await delay(1000);
  console.log(send_alice_2);
  t.deepEqual(send_alice_2[0].author, agent_pubkey_alice);
  t.deepEqual(send_alice_2[0].receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice_2[0].payload, "I was wondering if you were free today.");

  const send_alice_3 = await alice_cell.call('p2pmessage', 'send_message', message_3);
  await delay(1000);
  console.log(send_alice_3);
  t.deepEqual(send_alice_3[0].author, agent_pubkey_alice);
  t.deepEqual(send_alice_3[0].receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice_3[0].payload, "Would you like to go out for coffee?");

  // bobby sends a message to alice
  console.log("Bobby sends a messages to Alice");
  const send_bobby_1 = await bobby_cell.call('p2pmessage', 'send_message', message_4);
  await delay(1000);
  console.log(send_bobby_1);
  t.deepEqual(send_bobby_1[0].author, agent_pubkey_bobby);
  t.deepEqual(send_bobby_1[0].receiver, agent_pubkey_alice);
  t.deepEqual(send_bobby_1[0].payload, "Hi, Alice!");

  console.log(send_alice_3[1].id)
  // bobby replies to a message of alice
  const message_5_as_reply = {
    receiver: agent_pubkey_alice,
    payload: "Sure! I would love to go out for coffee with you!",
    reply_to: send_alice_3[1].id
  }
  
  const reply_bobby = await bobby_cell.call('p2pmessage', 'send_message', message_5_as_reply);
  await delay(1000);
  console.log("bob replies to a message of alice");
  console.log(reply_bobby);
  t.deepEqual(reply_bobby[0].author, agent_pubkey_bobby);
  t.deepEqual(reply_bobby[0].receiver, agent_pubkey_alice);
  t.deepEqual(reply_bobby[0].payload, "Sure! I would love to go out for coffee with you!");

  // carly sends 2 messages to bobby
  console.log("Carly sends three messages to Bobby");
  const send_carly_1 = await carly_cell.call('p2pmessage', 'send_message', message_6);
  await delay(1000);
  console.log(send_carly_1);
  t.deepEqual(send_carly_1[0].author, agent_pubkey_carly);
  t.deepEqual(send_carly_1[0].receiver, agent_pubkey_bobby);
  t.deepEqual(send_carly_1[0].payload, "Hi, Bobby!");

  const send_carly_2 = await carly_cell.call('p2pmessage', 'send_message', message_7);
  await delay(1000);
  console.log(send_carly_2);
  t.deepEqual(send_carly_2[0].author, agent_pubkey_carly);
  t.deepEqual(send_carly_2[0].receiver, agent_pubkey_bobby);
  t.deepEqual(send_carly_2[0].payload, "I have an extra ticket to a movie later. Would you want to come with me?");

  // bobby sends a messages to carly
  console.log("Bobby sends a message to Carly");
  const send_bobby_3 = await bobby_cell.call('p2pmessage', 'send_message', message_8);
  await delay(1000);
  console.log(send_bobby_3);
  t.deepEqual(send_bobby_3[0].author, agent_pubkey_bobby);
  t.deepEqual(send_bobby_3[0].receiver, agent_pubkey_carly);
  t.deepEqual(send_bobby_3[0].payload, "Hey, Carly. I'm sorry but I already made plans. maybe some other time?");

  // alice sends a message to bobby
  console.log("Alice sends a message to bobby");
  const send_alice_4 = await alice_cell.call('p2pmessage', 'send_message', message_9);
  await delay(1000);
  console.log(send_alice_4);
  t.deepEqual(send_alice_4[0].author, agent_pubkey_alice);
  t.deepEqual(send_alice_4[0].receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice_4[0].payload, "Great! I'll see you later!");

  // alice gets her latest messages
  const alice_latest_messages_1 = await alice_cell.call('p2pmessage', 'get_latest_messages', 1);
  await delay(1000);
  console.log("alice gets 1 latest message");
  for (var agent_key in alice_latest_messages_1[0]) {
    t.deepEqual(agent_key, agent_pubkey_bobby_string);
    t.deepEqual(alice_latest_messages_1[0][agent_key].length, 1);
    console.log("the messages with this agent are:")
    console.log(alice_latest_messages_1[0][agent_key])
    for (var message_key in alice_latest_messages_1[1]) {
      console.log("the message contents and receipts for this message hash are:")
      console.log(alice_latest_messages_1[1][message_key])
      for (var receipt_key in alice_latest_messages_1[2]) {
        console.log("the receipts for this message are:")
        console.log(alice_latest_messages_1[2][receipt_key])
      }
    }
  }

  const alice_latest_messages_2 = await alice_cell.call('p2pmessage', 'get_latest_messages', 2);
  await delay(1000);
  console.log("alice gets 2 latest message");
  for (var agent_key in alice_latest_messages_2[0]) {
    t.deepEqual(agent_key, agent_pubkey_bobby_string);
    t.deepEqual(alice_latest_messages_2[0][agent_key].length, 2);
    console.log("the messages with this agent are:")
    console.log(alice_latest_messages_2[0][agent_key])
    for (var message_key in alice_latest_messages_2[1]) {
      console.log("the message contents and receipts for this message hash are:")
      console.log(alice_latest_messages_2[1][message_key])
      for (var receipt_key in alice_latest_messages_2[2]) {
        console.log("the receipts for this message are:")
        console.log(alice_latest_messages_2[2][receipt_key])
      }
    }
  }

  // bobby gets his latest message
  console.log("Bobby gets his latest messaages (size 3)")
  const bobby_latest_messages_1 = await bobby_cell.call('p2pmessage', 'get_latest_messages', 1);
  await delay(1000);

  // bobby gets the next batch of messages from alice
  console.log("Bobby gets the next batch (size 1) of messages from Alice")
  const last_message_alice_hash_string = bobby_latest_messages_1[0][agent_pubkey_alice_string];
  const last_message_alice = bobby_latest_messages_1[1][last_message_alice_hash_string];
  const last_message_alice_receipt_hash_string = last_message_alice[1][0];
  const last_message_alice_receipt_id = bobby_latest_messages_1[2][last_message_alice_receipt_hash_string].id
  
  const batch_filter_alice = {
    conversant: agent_pubkey_alice,
    batch_size: 3,
    last_fetched_timestamp: last_message_alice[0].time_sent,
    last_fetched_message_id: last_message_alice_receipt_id
  }

  const bobby_next_batch_1 = await bobby_cell.call('p2pmessage', 'get_next_batch_messages', batch_filter_alice);
  await delay(1000);

  for (var agent_key in bobby_next_batch_1[0]) {
    t.deepEqual(agent_key, agent_pubkey_alice_string);
    t.deepEqual(bobby_next_batch_1[0][agent_key].length, 3);
    console.log("the messages with this agent are:")
    console.log(bobby_next_batch_1[0][agent_key])
    for (var message_key in bobby_next_batch_1[1]) {
      console.log("the message contents and receipts for this message hash are:")
      console.log(bobby_next_batch_1[1][message_key])
      for (var receipt_key in bobby_next_batch_1[2]) {
        console.log("the receipts for this message are:")
        console.log(bobby_next_batch_1[2][receipt_key])
      }
    }
  }

  // bobby gets the next batch of messages from carly
  console.log("Bobby gets the next batch (size 2) of messages from Carly")
  const last_message_carly_hash_string = bobby_latest_messages_1[0][agent_pubkey_carly_string];
  const last_message_carly = bobby_latest_messages_1[1][last_message_carly_hash_string];
  const last_message_carly_receipt_hash_string = last_message_carly[1][0];
  const last_message_carly_receipt_id = bobby_latest_messages_1[2][last_message_carly_receipt_hash_string].id
  
  const batch_filter_carly = {
    conversant: agent_pubkey_carly,
    batch_size: 2,
    last_fetched_timestamp: last_message_carly[0].time_sent,
    last_fetched_message_id: last_message_carly_receipt_id
  }

  const bobby_next_batch_2 = await bobby_cell.call('p2pmessage', 'get_next_batch_messages', batch_filter_carly);
  await delay(1000);

  for (var agent_key in bobby_next_batch_2[0]) {
    t.deepEqual(agent_key, agent_pubkey_carly_string);
    t.deepEqual(bobby_next_batch_2[0][agent_key].length, 2);
    console.log("the messages with this agent are:")
    console.log(bobby_next_batch_2[0][agent_key])
    for (var message_key in bobby_next_batch_2[1]) {
      console.log("the message contents and receipts for this message hash are:")
      console.log(bobby_next_batch_2[1][message_key])
      for (var receipt_key in bobby_next_batch_2[2]) {
        console.log("the receipts for this message are:")
        console.log(bobby_next_batch_2[2][receipt_key])
      }
    }
  }

  // carly gets her messages with alice for today
  const today = [ Math.floor(Date.now() / 1000), 0 ]

  console.log("Carly gets her messages with Alice for today");
  const timestamp_filter_alice = {
    conversant: agent_pubkey_alice,
    date: today
  }

  const alice_today = await carly_cell.call('p2pmessage', 'get_messages_by_agent_by_timestamp', timestamp_filter_alice);
  await delay(1000);
  
  for (var agent_key in alice_today[0]) {
    t.deepEqual(agent_key, agent_pubkey_alice_string);
    t.deepEqual(alice_today[0][agent_key].length, 0);
    console.log("the messages with this agent are:")
    console.log(alice_today[0][agent_key])
    for (var message_key in alice_today[1]) {
      console.log("the message contents and receipts for this message hash are:")
      console.log(alice_today[1][message_key])
      for (var receipt_key in alice_today[2]) {
        console.log("the receipts for this message are:")
        console.log(alice_today[2][receipt_key])
      }
    }
  }

  // carly gets her messages with bobby for today
  console.log("Carly gets her messages with Bobby for today");
  const timestamp_filter_bobby = {
    conversant: agent_pubkey_bobby,
    date: today
  }

  const bobby_today = await carly_cell.call('p2pmessage', 'get_messages_by_agent_by_timestamp', timestamp_filter_bobby);
  await delay(1000);

  for (var agent_key in bobby_today[0]) {
    t.deepEqual(agent_key, agent_pubkey_bobby_string);
    t.deepEqual(bobby_today[0][agent_key].length, 3);
    console.log("the messages with this agent are:")
    console.log(bobby_today[0][agent_key])
    for (var message_key in bobby_today[1]) {
      console.log("the message contents and receipts for this message hash are:")
      console.log(bobby_today[1][message_key])
      for (var receipt_key in bobby_today[2]) {
        console.log("the receipts for this message are:")
        console.log(bobby_today[2][receipt_key])
      }
    }
  }

});

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


//   const alice_cell = alice_happ.cells[0];
//   const bobby_cell = bobby_happ.cells[0];

//   const agent_pubkey_alice = alice_happ.agent
//   const agent_pubkey_bobby = bobby_happ.agent

//   console.log(agent_pubkey_alice)
//   console.log(agent_pubkey_bobby)

//   const message_async_1 = {
//     receiver: agent_pubkey_alice,
//     payload: "Read this when you get back online",
//     reply_to: null
//   }

//   const message_async_2 = {
//     receiver: agent_pubkey_alice,
//     payload: "I have read your message",
//     reply_to: null
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

orchestrator.run();

