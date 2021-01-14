import {
  Config,
  InstallAgentsHapps,
  Orchestrator,
  TransportConfigType,
} from "@holochain/tryorama";
import { Base64 } from "js-base64";
import path from "path";

const network = {
  transport_pool: [
    {
      type: TransportConfigType.Quic,
    },
  ],
  bootstrap_service: "https://bootstrap.holo.host",
};
const conductorConfig = Config.gen({ network });

const p2pmessagedna = path.join(__dirname, "../../p2pmessage.dna.gz");
const installation: InstallAgentsHapps = [[[p2pmessagedna]]];

const orchestrator = new Orchestrator();

const delay = (ms) => new Promise((r) => setTimeout(r, ms));

function get_batch_messages_on_conversation(messagerange) {
  return (conductor, caller) =>
    conductor.call(
      caller,
      "p2pmessage",
      "get_batch_messages_on_conversation",
      messagerange
    );
}

function send_message(message) {
  return (cell) => cell.call("p2pmessage", "send_message", message);
}

export function serializeHash(hash) {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

orchestrator.registerScenario("remote call", async (s, t) => {
  const [alice, bobby, carly] = await s.players([
    conductorConfig,
    conductorConfig,
    conductorConfig,
  ]);
  const [[alice_happ]] = await alice.installAgentsHapps(installation);
  const [[bobby_happ]] = await bobby.installAgentsHapps(installation);
  const [[carly_happ]] = await carly.installAgentsHapps(installation);

  let push_notifs = {};
  let messages = {};
  const handleSignal = (signal) => {
    const message = signal.data.payload.message;
    console.log("signal ello");
    push_notifs = { ...push_notifs, [JSON.stringify(message)]: message };
  };

  alice.setSignalHandler(handleSignal);
  bobby.setSignalHandler(handleSignal);
  carly.setSignalHandler(handleSignal);

  // await s.shareAllNodes([alice, bobby, carly]);

  // const [player]: Player[] = await s.players([conductorConfig]);

  // const aliceKey = await player.adminWs().generateAgentPubKey();

  // const dnas = [
  //   {
  //     path: p2pmessagedna,
  //     nick: `my_cell_nick`,
  //     properties: { progenitors: [serializeHash(aliceKey)] },
  //     membrane_proof: undefined,
  //   },
  // ];

  // const alice_happ = await player._installHapp({
  //   installed_app_id: `my_app:12345`, // my_app with some unique installed id value
  //   agent_key: aliceKey,
  //   dnas,
  // });
  // const bobby_happ = await player._installHapp({
  //   installed_app_id: `my_app:1234`, // my_app with some unique installed id value
  //   agent_key: await player.adminWs().generateAgentPubKey(),
  //   dnas,
  // });
  // const carly_happ = await player._installHapp({
  //   installed_app_id: `my_app:123`, // my_app with some unique installed id value
  //   agent_key: await player.adminWs().generateAgentPubKey(),
  //   dnas,
  // });qweqweqw

  const alice_cell = alice_happ.cells[0];
  const bobby_cell = bobby_happ.cells[0];
  const carly_cell = carly_happ.cells[0];

  const agent_pubkey_alice = alice_happ.agent;
  const agent_pubkey_bobby = bobby_happ.agent;
  const agent_pubkey_carly = carly_happ.agent;

  console.log(agent_pubkey_alice);
  console.log(agent_pubkey_bobby);
  console.log(agent_pubkey_carly);

  const message = {
    receiver: agent_pubkey_bobby,
    payload: "Hello world",
    reply_to: null,
  };

  const message_2 = {
    receiver: agent_pubkey_bobby,
    payload: "Hello world 2",
    reply_to: null,
  };

  const message_3 = {
    receiver: agent_pubkey_alice,
    payload: "Hello alice",
    reply_to: null,
  };

  const message_4 = {
    receiver: agent_pubkey_alice,
    payload: "I am Carly",
    reply_to: null,
  };

  const message_late_1 = {
    receiver: agent_pubkey_alice,
    payload: "Hello again",
    reply_to: null,
  };

  const message_late_2 = {
    receiver: agent_pubkey_alice,
    payload: "Am I bothering you",
    reply_to: null,
  };

  const message_async = {
    receiver: agent_pubkey_bobby,
    payload: "Read this when you get back online",
    reply_to: null,
  };

  // alice sends a message to bob
  const send_alice = await alice_cell.call(
    "p2pmessage",
    "send_message",
    message
  );

  messages = { ...messages, [JSON.stringify(send_alice)]: send_alice };
  await delay(1000);
  console.log("alice sends a message to bob");
  console.log(send_alice);
  t.deepEqual(send_alice.author, agent_pubkey_alice);
  t.deepEqual(send_alice.receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice.payload, "Hello world");
  t.deepEqual(send_alice.status, { Delivered: null });

  // alice sends another message to bob
  const send_alice_2 = await alice_cell.call(
    "p2pmessage",
    "send_message",
    message_2
  );
  messages = { ...messages, [JSON.stringify(send_alice_2)]: send_alice_2 };

  await delay(1000);
  console.log("alice sends a message to bob");
  console.log(send_alice_2);
  t.deepEqual(send_alice_2.author, agent_pubkey_alice);
  t.deepEqual(send_alice_2.receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice_2.payload, "Hello world 2");
  t.deepEqual(send_alice_2.status, { Delivered: null });

  // bob replies to a message of alice
  const replied_message = send_alice_2;
  const reply = {
    receiver: agent_pubkey_alice,
    payload: "Hello back",
    reply_to: replied_message,
  };

  const reply_bobby = await bobby_cell.call(
    "p2pmessage",
    "send_message",
    reply
  );

  messages = { ...messages, [JSON.stringify(reply_bobby)]: reply_bobby };

  await delay(1000);
  console.log("bob replies to a message of alice");
  console.log(reply_bobby);
  t.deepEqual(reply_bobby.author, agent_pubkey_bobby);
  t.deepEqual(reply_bobby.receiver, agent_pubkey_alice);
  t.deepEqual(reply_bobby.payload, "Hello back");
  t.deepEqual(reply_bobby.status, { Delivered: null });

  // alice gets all messages in her source chain
  const all_messages_alice = await alice_cell.call(
    "p2pmessage",
    "get_all_messages",
    null
  );
  await delay(1000);
  console.log("alice gets all messages in her source chain");
  console.log(all_messages_alice);
  t.deepEqual(all_messages_alice.length, 3);

  // bob gets all messages in his source chain
  const all_messages_bobby = await bobby_cell.call(
    "p2pmessage",
    "get_all_messages",
    null
  );
  await delay(1000);
  console.log("bob gets all messages in his source chain");
  console.log(all_messages_bobby);
  t.deepEqual(all_messages_bobby.length, 3);

  // carly sends a message to alice
  const send_carly = await carly_cell.call(
    "p2pmessage",
    "send_message",
    message_3
  );

  messages = { ...messages, [JSON.stringify(send_carly)]: send_carly };

  await delay(1000);
  console.log("carly sends a message to alice");
  console.log(send_carly);
  t.deepEqual(send_carly.author, agent_pubkey_carly);
  t.deepEqual(send_carly.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly.payload, "Hello alice");
  t.deepEqual(send_carly.status, { Delivered: null });

  // carly sends another message to alice
  const send_carly_2 = await carly_cell.call(
    "p2pmessage",
    "send_message",
    message_4
  );

  messages = { ...messages, [JSON.stringify(send_carly_2)]: send_carly_2 };

  await delay(1000);
  console.log("carly sends message to alice again");
  console.log(send_carly_2);
  t.deepEqual(send_carly_2.author, agent_pubkey_carly);
  t.deepEqual(send_carly_2.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly_2.payload, "I am Carly");
  t.deepEqual(send_carly_2.status, { Delivered: null });

  // alice has messages from bobby and carly in her source chain
  const messages_in_alice_from_both = await alice_cell.call(
    "p2pmessage",
    "get_all_messages_from_addresses",
    [agent_pubkey_bobby, agent_pubkey_carly]
  );
  await delay(1000);
  console.log("alice gets her messages from bobby and carly");
  console.log(messages_in_alice_from_both);
  t.deepEqual(messages_in_alice_from_both.length, 2);
  t.deepEqual(
    messages_in_alice_from_both[0].messages.length +
      messages_in_alice_from_both[1].messages.length,
    3
  );

  // author order may be arbitrary so the following assertions can fail
  // t.deepEqual(messages_in_alice_from_both[0].messages.length, 2);
  // t.deepEqual(messages_in_alice_from_both[1].messages.length, 1);

  // message order is still arbitrary
  // t.deepEqual(messages_in_alice_from_both[0].messages[0].payload, "Hello back");
  // t.deepEqual(messages_in_alice_from_both[1].messages[0].payload, "Hello alice");
  // t.deepEqual(messages_in_alice_from_both[1].messages[1].payload, "I am Carly");

  const send_carly_3 = await carly_cell.call(
    "p2pmessage",
    "send_message",
    message_late_1
  );
  messages = { ...messages, [JSON.stringify(send_carly_3)]: send_carly_3 };

  await delay(1000);
  console.log("carly sends message to alice again");
  console.log(send_carly_3);
  t.deepEqual(send_carly_3.author, agent_pubkey_carly);
  t.deepEqual(send_carly_3.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly_3.payload, "Hello again");
  t.deepEqual(send_carly_3.status, { Delivered: null });

  await delay(10000);

  const send_carly_4 = await carly_cell.call(
    "p2pmessage",
    "send_message",
    message_late_2
  );

  messages = { ...messages, [JSON.stringify(send_carly_4)]: send_carly_4 };

  await delay(1000);
  console.log("carly sends message to alice again");
  console.log(send_carly_4);
  t.deepEqual(send_carly_4.author, agent_pubkey_carly);
  t.deepEqual(send_carly_4.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly_4.payload, "Am I bothering you");
  t.deepEqual(send_carly_4.status, { Delivered: null });

  const last_message = {
    author: agent_pubkey_carly,
    last_message_timestamp_seconds: send_carly_4.time_sent[0] + 2,
  };

  const batch_messages = await alice_cell.call(
    "p2pmessage",
    "get_batch_messages_on_conversation",
    last_message
  );
  await delay(1000);
  console.log("alice batch fetches her messages");
  console.log(batch_messages);
  t.deepEqual(batch_messages.length, 1);

  //bobby goes offline
  await bobby.shutdown();

  const send_async_alice = await alice_cell.call(
    "p2pmessage",
    "send_message",
    message_async
  );
  await delay(1000);
  console.log("alice sends a message to offline bob");
  console.log(send_async_alice);
  t.deepEqual(send_async_alice.author, agent_pubkey_alice);
  t.deepEqual(send_async_alice.receiver, agent_pubkey_bobby);
  t.deepEqual(send_async_alice.payload, "Read this when you get back online");
  t.deepEqual(send_async_alice.status, { Sent: null });

  // bobby comes back online
  await bobby.startup();

  await delay(10000);
  console.log("\n\n\n" + messages);
  console.log("\n\n\n" + push_notifs);

  t.deepEqual(messages, push_notifs);
  // const async_messages_bobby = await bobby_cell.call(
  //   "p2pmessage",
  //   "fetch_async_messages",
  //   null
  // );

  // console.log(async_messages_bobby)
});

orchestrator.registerScenario("Test emit_signal", async (s, t) => {
  const [alice, bobby, carly] = await s.players([
    conductorConfig,
    conductorConfig,
    conductorConfig,
  ]);

  const [[alice_happ]] = await alice.installAgentsHapps(installation);
  const [[bobby_happ]] = await bobby.installAgentsHapps(installation);
  const [[carly_happ]] = await carly.installAgentsHapps(installation);
  let index = 0;
  const alice_cell = alice_happ.cells[0];
  const carly_cell = carly_happ.cells[0];

  const agent_pubkey_alice = alice_happ.agent;
  const agent_pubkey_carly = carly_happ.agent;

  bobby.setSignalHandler((signal) => {
    t.deepEqual(signal.data.payload, signalResults[index]);
    index += 1;
  });
  const signalResults = [
    {
      is_typing: true,
      kind: "typing",
      agent: agent_pubkey_alice,
    },
    {
      is_typing: true,
      kind: "typing",
      agent: agent_pubkey_carly,
    },
    {
      is_typing: false,
      kind: "typing",
      agent: agent_pubkey_alice,
    },
    {
      is_typing: false,
      kind: "typing",
      agent: agent_pubkey_carly,
    },
  ];

  await alice_cell.call("p2pmessage", "typing", {
    agent: bobby_happ.agent,
    is_typing: true,
  });
  await carly_cell.call("p2pmessage", "typing", {
    agent: bobby_happ.agent,
    is_typing: true,
  });
  await alice_cell.call("p2pmessage", "typing", {
    agent: bobby_happ.agent,
    is_typing: false,
  });
  await carly_cell.call("p2pmessage", "typing", {
    agent: bobby_happ.agent,
    is_typing: false,
  });
});

orchestrator.run();
