import { AppSignal } from "@holochain/conductor-api";
import { Orchestrator, Player } from "@holochain/tryorama";
import { delay, extractPayloadFromSignal, dateToTimestamp } from "../utils";
import { Installables } from "../types";

const handleTypeSignal = (signal: AppSignal) => () =>
  extractPayloadFromSignal(signal);

const evaluateReceipts = (receipts, resultReceipts) =>
  resultReceipts.every((resultReceipt) =>
    receipts
      .map((receipt) => JSON.stringify(receipt))
      .includes(JSON.stringify(resultReceipt))
  );

function sendMessage(message) {
  return (conductor) => conductor.call("p2pmessage", "send_message", message);
}

function readMessage(read_receipt_input) {
  return (conductor) =>
    conductor.call("p2pmessage", "read_message", read_receipt_input);
}

function getLatestMessages(batch_size) {
  return (conductor) =>
    conductor.call("p2pmessage", "get_latest_messages", batch_size);
}

const receipts = async (conductorConfig, installation: Installables) => {
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario("Receipts test", async (s, t) => {
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
          payload: { payload: "Hi Bob!" },
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_alice,
        payload: {
          type: "TEXT",
          payload: { payload: "Hey Alice" },
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "How are ya?" },
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_alice,
        payload: {
          type: "TEXT",
          payload: { payload: "I'm fine!! You?" },
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "TEXT",
          payload: { payload: "I'm fine as well 😄" },
        },
        replyTo: null,
      },
    ];

    const message_1_result = await sendMessage(messages[0])(alice_cell);
    await delay(1000);

    const message_2_result = await sendMessage(messages[1])(bobby_cell);
    await delay(1000);

    const message_3_result = await sendMessage(messages[2])(alice_cell);
    await delay(1000);

    const message_4_result = await sendMessage(messages[3])(bobby_cell);
    await delay(1000);

    const message_5_result = await sendMessage(messages[4])(alice_cell);
    await delay(1000);

    // check delivered receipts
    t.deepEqual("delivered", message_1_result[1][1].status.status);
    t.deepEqual("delivered", message_2_result[1][1].status.status);
    t.deepEqual("delivered", message_3_result[1][1].status.status);
    t.deepEqual("delivered", message_4_result[1][1].status.status);
    t.deepEqual("delivered", message_5_result[1][1].status.status);

    const dateRead = dateToTimestamp(new Date());

    const p2p_message_1 = message_1_result[0];
    const p2p_delivered_receipt_1 = message_1_result[1];
    const p2p_message_1_hash = p2p_message_1[0];

    const p2p_message_2 = message_2_result[0];
    const p2p_delivered_receipt_2 = message_2_result[1];
    const p2p_message_2_hash = p2p_message_2[0];

    const p2p_message_3 = message_3_result[0];
    const p2p_delivered_receipt_3 = message_3_result[1];
    const p2p_message_3_hash = p2p_message_3[0];

    const p2p_message_4 = message_4_result[0];
    const p2p_delivered_receipt_4 = message_1_result[1];
    const p2p_message_4_hash = p2p_message_4[0];

    const p2p_message_5 = message_5_result[0];
    const p2p_delivered_receipt_5 = message_5_result[1];
    const p2p_message_5_hash = p2p_message_5[0];

    const read_message_1 = await readMessage({
      message_hashes: [p2p_message_1_hash],
      sender: agent_pubkey_alice,
      timestamp: dateRead,
    })(bobby_cell);
    await delay(1000);

    const read_message_3_5 = await readMessage({
      message_hashes: [p2p_message_3_hash, p2p_message_5_hash],
      sender: agent_pubkey_alice,
      timestamp: dateRead,
    })(bobby_cell);
    await delay(1000);

    for (let receipt_hash in read_message_1) {
      t.deepEqual("read", read_message_1[receipt_hash].status.status);
      t.deepEqual(dateRead, read_message_1[receipt_hash].status.timestamp);
    }

    for (let receipt_hash in read_message_3_5) {
      console.log(receipt_hash);
      t.deepEqual("read", read_message_3_5[receipt_hash].status.status);
      t.deepEqual(dateRead, read_message_3_5[receipt_hash].status.timestamp);
    }

    const fetchedMessages = await getLatestMessages(5)(alice_cell);
    await delay(1000);

    let bobbyMessages = fetchedMessages[1];
    let bobbyReceipts = fetchedMessages[2];

    const delivered_messages = [
      p2p_message_1_hash,
      p2p_message_2_hash,
      p2p_message_3_hash,
      p2p_message_4_hash,
      p2p_message_5_hash,
    ];
    const read_messages = [
      p2p_message_1_hash,
      p2p_message_3_hash,
      p2p_message_5_hash,
    ];

    let read_messages_in_receipts: any[] = [];
    let delivered_messages_in_receipts: any[] = [];

    for (let receipt_hash in bobbyReceipts) {
      let receipt = bobbyReceipts[receipt_hash];
      let message_ids_in_receipt = [...receipt.id];
      message_ids_in_receipt.map((id) => {
        console.log(id);
        if (receipt.status.status === "delivered") {
          delivered_messages_in_receipts.push(id);
        } else {
          read_messages_in_receipts.push(id);
        }
      });
    }

    t.deepEqual(
      JSON.stringify(delivered_messages.sort()),
      JSON.stringify(delivered_messages_in_receipts.sort())
    );
    t.deepEqual(
      JSON.stringify(read_messages.sort()),
      JSON.stringify(read_messages_in_receipts.sort())
    );

    // const receipts = Object.values(fetchedMessages[2]);
    // const resultReceipts = [read_message_1, read_message_2, read_message_3].map(
    //   (resultReceipt) => Object.values(resultReceipt)[0]
    // );

    // t.deepEqual(true, evaluateReceipts(receipts, resultReceipts));
    // t.deepEqual(true, evaluateReceipts(receiptsFromSignal, resultReceipts));
  });

  orchestrator.run();
};

export default receipts;
