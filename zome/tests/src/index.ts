import { Orchestrator } from '@holochain/tryorama'
import { Config } from '@holochain/tryorama'

const orchestrator = new Orchestrator()

const delay = (ms) => new Promise((r) => setTimeout(r, ms));

const config = Config.gen({
  alice: Config.dna('../p2pmessage.dna.gz', null),
  bobby: Config.dna('../p2pmessage.dna.gz', null),
  carly: Config.dna('../p2pmessage.dna.gz', null)
})

function send_message(message) {
    return (conductor, caller) =>
      conductor.call(caller, "p2pmessage", "send_message", message);
};
  
function receive_message() {
  return (conductor, caller) =>
    conductor.call(caller, "p2pmessage", "receive_message", null);
};

function reply_to_message(reply) {
  return (conductor, caller) =>
    conductor.call(caller, "p2pmessage", "reply_to_message", reply);
};

function get_all_messages() {
  return (conductor, caller) =>
    conductor.call(caller, "p2pmessage", "get_all_messages", null);
}

function get_all_messages_from_addresses(agentlist) {
  return (conductor, caller) =>
    conductor.call(caller, "p2pmessage", "get_all_messages_from_addresses", agentlist);
}

function get_batch_messages_on_conversation(messagerange) {
  return (conductor, caller) => 
  conductor.call(caller, "p2pmessage", "get_batch_messages_on_conversation", messagerange);
}

orchestrator.registerScenario("p2p messaging", async (s, t) => {
  const { conductor } = await s.players({ conductor: config });
  await conductor.spawn();

  const [dna_hash_1, agent_pubkey_alice] = conductor.cellId('alice');
  const [dna_hash_2, agent_pubkey_bobby] = conductor.cellId('bobby');
  const [dna_hash_3, agent_pubkey_carly] = conductor.cellId('carly');
  console.log("alice address");
  console.log(agent_pubkey_alice);
  console.log("bobby address");
  console.log(agent_pubkey_bobby);
  console.log("carly address");
  console.log(agent_pubkey_carly);

  const message = {
      receiver: agent_pubkey_bobby,
      payload: "Hello world"
  };

  const message_2 = {
      receiver: agent_pubkey_alice,
      payload: "Hello back"
  }

  const message_3 = {
      receiver: agent_pubkey_alice,
      payload: "Hello alice"
  }

  const message_4 = {
      receiver: agent_pubkey_alice,
      payload: "I am Carly"
  }

  const message_late_1 = {
      receiver: agent_pubkey_alice,
      payload: "Hello again"
  }

  const message_late_2 = {
      receiver: agent_pubkey_alice,
      payload: "Am I bothering you"
  }

  // alice sends a message to bob
  const send_alice = await send_message(message)(conductor, 'alice');
  await delay(1000);
  console.log("alice sends a message to bob");
  console.log(send_alice);
  t.deepEqual(send_alice.author, agent_pubkey_alice);
  t.deepEqual(send_alice.receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice.payload, "Hello world");

  // alice sends another message to bob
  const send_alice_2 = await send_message(message)(conductor, 'alice');
  await delay(1000);
  console.log("alice sends a message to bob");
  console.log(send_alice_2);
  t.deepEqual(send_alice_2.author, agent_pubkey_alice);
  t.deepEqual(send_alice_2.receiver, agent_pubkey_bobby);
  t.deepEqual(send_alice_2.payload, "Hello world");

  const message_1_reply = {
    replied_message: send_alice_2,
    reply: "Hello back reply"
  }

  // bob replies to alice
  const send_bobby = await reply_to_message(message_1_reply)(conductor, 'bobby');
  await delay(1000);
  console.log("bob replies to a message of alice");
  console.log(send_bobby);
  t.deepEqual(send_bobby.author, agent_pubkey_bobby);
  t.deepEqual(send_bobby.receiver, agent_pubkey_alice);
  t.deepEqual(send_bobby.payload, "Hello back reply");
  console.log(send_bobby.reply_to);

  // alice gets all messages in her source chain
  const all_messages_alice = await get_all_messages()(conductor, 'alice');
  await delay(1000);
  console.log("alice gets all messages in her source chain");
  console.log(all_messages_alice);
  t.deepEqual(all_messages_alice.length, 3);

  // bob gets all messages in his source chain
  const all_messages_bobby = await get_all_messages()(conductor, 'bobby');
  await delay(1000);
  console.log("bob gets all messages in his source chain");
  console.log(all_messages_bobby);
  t.deepEqual(all_messages_bobby.length, 3);

  // carly sends a message to alice
  const send_carly = await send_message(message_3)(conductor, 'carly');
  await delay(1000);
  console.log("carly sends a message to alice");
  console.log(send_carly);
  t.deepEqual(send_carly.author, agent_pubkey_carly);
  t.deepEqual(send_carly.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly.payload, "Hello alice");

  // carly sends another message to alice
  const send_carly_2 = await send_message(message_4)(conductor, 'carly');
  await delay(1000);
  console.log("carly sends message to alice again");
  console.log(send_carly_2);
  t.deepEqual(send_carly_2.author, agent_pubkey_carly);
  t.deepEqual(send_carly_2.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly_2.payload, "I am Carly");

  // alice has messages from bobby and carly in her source chain
  const messages_in_alice_from_both = await get_all_messages_from_addresses([agent_pubkey_bobby, agent_pubkey_carly])(conductor, 'alice');
  await delay(1000);
  console.log("alice gets her messages from bobby and carly");
  console.log(messages_in_alice_from_both);
  t.deepEqual(messages_in_alice_from_both.length, 2);
  t.deepEqual(messages_in_alice_from_both[0].messages.length + messages_in_alice_from_both[1].messages.length, 3);

  // author order may be arbitrary so the following assertions can fail
  // t.deepEqual(messages_in_alice_from_both[0].messages.length, 2);
  // t.deepEqual(messages_in_alice_from_both[1].messages.length, 1);

  // message order is still arbitrary
  // t.deepEqual(messages_in_alice_from_both[0].messages[0].payload, "Hello back");
  // t.deepEqual(messages_in_alice_from_both[1].messages[0].payload, "Hello alice");
  // t.deepEqual(messages_in_alice_from_both[1].messages[1].payload, "I am Carly");

  const send_carly_3 = await send_message(message_late_1)(conductor, 'carly');
  await delay(1000);
  console.log("carly sends message to alice again");
  console.log(send_carly_3);
  t.deepEqual(send_carly_3.author, agent_pubkey_carly);
  t.deepEqual(send_carly_3.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly_3.payload, "Hello again");

  await delay(10000);

  const send_carly_4 = await send_message(message_late_2)(conductor, 'carly');
  await delay(1000);
  console.log("carly sends message to alice again");
  console.log(send_carly_4);
  t.deepEqual(send_carly_4.author, agent_pubkey_carly);
  t.deepEqual(send_carly_4.receiver, agent_pubkey_alice);
  t.deepEqual(send_carly_4.payload, "Am I bothering you");

  const last_message = {
      author: agent_pubkey_carly,
      last_message_timestamp_seconds: send_carly_4.time_sent[0]+2
  };

  const batch_messages = await get_batch_messages_on_conversation(last_message)(conductor, 'alice');
  await delay(1000);
  console.log("alice batch fetches her messages");
  console.log(batch_messages);
  t.deepEqual(batch_messages.length, 1);
});


orchestrator.run()