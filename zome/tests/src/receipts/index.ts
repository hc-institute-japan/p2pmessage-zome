import { AppSignal } from "@holochain/conductor-api";
import { FunctionType } from "../types";
import { delay, extractPayloadFromSignal } from "../utils";

const handleTypeSignal = (signal: AppSignal) => () =>
  extractPayloadFromSignal(signal);
const receipts: FunctionType = async (
  orchestrator,
  conductorConfig,
  installation
) => {
  const evaluateReceipts = (receipts, resultReceipts) =>
    resultReceipts.every((resultReceipt) =>
      receipts
        .map((receipt) => JSON.stringify(receipt))
        .includes(JSON.stringify(resultReceipt))
    );

  orchestrator.registerScenario("Typing signal test", async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);
    const [[alice_happ]] = await alice.installAgentsHapps(installation);
    const [[bobby_happ]] = await bob.installAgentsHapps(installation);
    const alice_cell = alice_happ.cells[0];
    const bobby_cell = bobby_happ.cells[0];

    await bobby_cell.call("p2pmessage", "init");
    await alice_cell.call("p2pmessage", "init");
    let receiptsFromSignal: any[] = [];
    alice.setSignalHandler((signal) => {
      let result = handleTypeSignal(signal)();
      let arr = Object.values(result);
      if (arr.length > 1) receiptsFromSignal.push(arr[1]);
    });

    const agent_pubkey_bobby = bobby_happ.agent;
    const agent_pubkey_alice = alice_happ.agent;
    let messages = [
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "text",
          payload: "Hi Bob!",
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_alice,
        payload: {
          type: "text",
          payload: "Hey Alice",
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "text",
          payload: "How are ya?",
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_alice,
        payload: {
          type: "text",
          payload: "I'm fine!! You?",
        },
        replyTo: null,
      },
      {
        receiver: agent_pubkey_bobby,
        payload: {
          type: "text",
          payload: "I'm fine as well 😄",
        },
        replyTo: null,
      },
    ];

    const message_1_result = await alice_cell.call(
      "p2pmessage",
      "send_message",
      messages[0]
    );

    await delay();

    const message_2_result = await bobby_cell.call(
      "p2pmessage",
      "send_message",
      messages[1]
    );

    await delay();

    const message_3_result = await alice_cell.call(
      "p2pmessage",
      "send_message",
      messages[2]
    );

    await delay();

    const message_4_result = await bobby_cell.call(
      "p2pmessage",
      "send_message",
      messages[3]
    );

    await delay();

    const message_5_result = await alice_cell.call(
      "p2pmessage",
      "send_message",
      messages[4]
    );

    await delay();

    const read_message_1 = await bobby_cell.call("p2pmessage", "read_message", {
      receipt: {
        id: message_1_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    });

    await delay();
    const read_message_2 = await bobby_cell.call("p2pmessage", "read_message", {
      receipt: {
        id: message_3_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    });

    await delay();
    const read_message_3 = await bobby_cell.call("p2pmessage", "read_message", {
      receipt: {
        id: message_5_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    });

    await delay();

    await bobby_cell.call("p2pmessage", "read_message", {
      receipt: {
        id: message_5_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    });
    await delay();

    const fetchedMessages = await bobby_cell.call(
      "p2pmessage",
      "get_latest_messages",
      10
    );

    await delay();

    const receipts = Object.values(fetchedMessages[2]);
    const resultReceipts = [read_message_1, read_message_2, read_message_3].map(
      (resultReceipt) => Object.values(resultReceipt)[0]
    );

    t.deepEqual(true, evaluateReceipts(receipts, resultReceipts));
    t.deepEqual(true, evaluateReceipts(receiptsFromSignal, resultReceipts));
  });
};

export default receipts;
