import { AppSignal } from "@holochain/conductor-api";
import { Orchestrator, Player } from "@holochain/tryorama";
import { delay, extractPayloadFromSignal } from "../utils";
import { Installables } from "../types";

const handleTypeSignal = (signal: AppSignal) => () =>
  extractPayloadFromSignal(signal);

const evaluateReceipts = (receipts, resultReceipts) =>
  resultReceipts.every((resultReceipt) =>
    receipts
      .map((receipt) => JSON.stringify(receipt))
      .includes(JSON.stringify(resultReceipt))
  );
  
function sendMessage(message){
  return (conductor)=>
    conductor.call("p2pmessage", "send_message", message);
}

function readMessage(read_receipt_input){
  return (conductor)=>
    conductor.call("p2pmessage", "read_message", read_receipt_input);
}

function getLatestMessages(batch_size){
  return (conductor)=>
    conductor.call("p2pmessage","get_latest_messages",batch_size);
}

const receipts = async ( conductorConfig, installation:Installables) => {
  
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario("Typing signal test", async (s, t) => {

    const [alice, bob]:Player[] = await s.players([conductorConfig, conductorConfig]);

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

    const read_message_1 = await readMessage({
      receipt: {
        id: message_1_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    })(bobby_cell);

    await delay(1000); 

    const read_message_2 = await readMessage({
      receipt: {
        id: message_3_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    })(bobby_cell);

    await delay(1000); 

    const read_message_3 = await readMessage({
      receipt: {
        id: message_5_result[1].id,
        status: {
          status: "read",
          timestamp: [new Date(2021, 1, 9).getTime(), 0],
        },
      },
      sender: agent_pubkey_alice,
    })(bobby_cell);

    await delay(1000);

    const fetchedMessages = await getLatestMessages(10)(bobby_cell);
    await delay(1000);

    const receipts = Object.values(fetchedMessages[2]);
    const resultReceipts = [read_message_1, read_message_2, read_message_3].map( (resultReceipt) => Object.values(resultReceipt)[0] );

    t.deepEqual(true, evaluateReceipts(receipts, resultReceipts));
    t.deepEqual(true, evaluateReceipts(receiptsFromSignal, resultReceipts));

  });

  orchestrator.run();
};

export default receipts;
