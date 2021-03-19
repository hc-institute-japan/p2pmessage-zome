import { Orchestrator, Player } from "@holochain/tryorama";
import { Base64 } from "js-base64";
import { Installables } from "../types";
import { delay } from "../utils";


function init(conductor) {
  return conductor.call("p2pmessage", "init",);
}

function sendMessage(message){
  return (conductor)=>
    conductor.call("p2pmessage", "send_message", message);
}

function getLatestMessages(batch_size){
  return (conductor)=>
    conductor.call("p2pmessage","get_latest_messages",batch_size);
}

function getNextBatchMessages(batch_filter){
  return (conductor)=>
    conductor.call("p2pmessage","get_next_batch_messages", batch_filter)
}

function getMessagesByAgentByTimestamp(timestamp_filter){
  return (conductor)=>
    conductor.call("p2pmessage","get_messages_by_agent_by_timestamp", timestamp_filter);
}
function strToUtf8Bytes(str) {
  const bytes: Array<number> = [];
  // const bytes = new Int8Array();
  for (let ii = 0; ii < str.length; ii++) {
    const code = str.charCodeAt(ii); // x00-xFFFF
    // bytes.push(code & 255, code >> 8); // low, high
    bytes.push(code & 255); // low, high
  }
  return bytes;
}

function hash_file(input_text) {
  let file_hash = require("crypto")
    .createHash("sha256")
    .update(input_text)
    .digest("hex");
  return file_hash;
}

//  MANUEL: UNUSED METHODS :

function utf8_to_str(a) {
  for (var i = 0, s = ""; i < a.length; i++) {
    var h = a[i].toString(16);
    if (h.length < 2) h = "0" + h;
    s += "%" + h;
  }
  return decodeURIComponent(s);
}
function serializeHash(hash) {
  return `u${Base64.fromUint8Array(hash, true)}`;
}


const messaging = async ( conductorConfig, installation:Installables ) => {

  let orchestrator = new Orchestrator();
   
  orchestrator.registerScenario("p2pmessage", async (s, t) => {

    const [alice,bobby,carly]:Player[] = await s.players([conductorConfig,conductorConfig,conductorConfig])

    const [[alice_happ]] = await alice.installAgentsHapps(installation.one);
    const [[bobby_happ]] = await bobby.installAgentsHapps(installation.one);
    const [[carly_happ]] = await carly.installAgentsHapps(installation.one);

    const alice_cell = alice_happ.cells[0];
    const bobby_cell = bobby_happ.cells[0];
    const carly_cell = carly_happ.cells[0];

    const agent_pubkey_alice = alice_happ.agent;
    const agent_pubkey_bobby = bobby_happ.agent;
    const agent_pubkey_carly = carly_happ.agent;

  
    var agent_pubkey_alice_string =
      "u" + agent_pubkey_alice.toString("base64");
    agent_pubkey_alice_string = agent_pubkey_alice_string.replace(/\//g, "_");
    agent_pubkey_alice_string = agent_pubkey_alice_string.replace(/\+/g, "-");

    var agent_pubkey_bobby_string =
      "u" + agent_pubkey_bobby.toString("base64");
    agent_pubkey_bobby_string = agent_pubkey_bobby_string.replace(/\//g, "_");
    agent_pubkey_bobby_string = agent_pubkey_bobby_string.replace(/\+/g, "-");

    var agent_pubkey_carly_string =
      "u" + agent_pubkey_carly.toString("base64");
    agent_pubkey_carly_string = agent_pubkey_carly_string.replace(/\//g, "_");
    agent_pubkey_carly_string = agent_pubkey_carly_string.replace(/\+/g, "-");

        
    // console.log(agent_pubkey_alice);
    // console.log(agent_pubkey_bobby);
    // console.log(agent_pubkey_carly);

    const message_1 = {
      receiver: agent_pubkey_bobby,
      payload: { type: "text", payload: "Hello, Bobby." },
      replyTo: null,
    };

    const message_2 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "text",
        payload: "I was wondering if you were free today.",
      },
      replyTo: null,
    };

    const message_3 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "text",
        payload: "Would you like to go out for coffee?",
      },
      replyTo: null,
    };

    const message_4 = {
      receiver: agent_pubkey_alice,
      payload: { type: "text", payload: "Hi, Alice!" },
      replyTo: null,
    };

    // reply in this sequence

    const message_6 = {
      receiver: agent_pubkey_bobby,
      payload: { type: "text", payload: "Hi, Bobby!" },
      replyTo: null,
    };

    const message_7 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "text",
        payload:
          "I have an extra ticket to a movie later. Would you want to come with me?",
      },
      replyTo: null,
    };

    const message_8 = {
      receiver: agent_pubkey_carly,
      payload: {
        type: "text",
        payload:
          "Hey, Carly. I'm sorry but I already made plans. maybe some other time?",
      },
      replyTo: null,
    };

    const message_9 = {
      receiver: agent_pubkey_bobby,
      payload: { type: "text", payload: "Great! I'll see you later!" },
      replyTo: null,
    };

    // alice sends 3 messages to bobby
    // console.log("Alice sends three messages to Bobby");

    const send_alice_1 = await sendMessage(message_1)(alice_cell);
    await delay(1000);

    // console.log(send_alice_1);
    t.deepEqual(send_alice_1[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_1[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_alice_1[0].payload.payload, "Hello, Bobby.");

    const send_alice_2 = await sendMessage(message_2)(alice_cell);
    await delay(1000);
    
    // console.log(send_alice_2);
    t.deepEqual(send_alice_2[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_2[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_alice_2[0].payload.payload, "I was wondering if you were free today.");

    const send_alice_3 = await sendMessage(message_3)(alice_cell);
    await delay(1000);

    // console.log(send_alice_3);
    t.deepEqual(send_alice_3[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_3[0].receiver, agent_pubkey_bobby);
    t.deepEqual( send_alice_3[0].payload.payload,"Would you like to go out for coffee?");

    // bobby sends a message to alice
    // console.log("Bobby sends a messages to Alice");
    
    const send_bobby_1 = await sendMessage(message_4)(bobby_cell);
    await delay(1000);
    
    // console.log(send_bobby_1);
    t.deepEqual(send_bobby_1[0].author, agent_pubkey_bobby);
    t.deepEqual(send_bobby_1[0].receiver, agent_pubkey_alice);
    t.deepEqual(send_bobby_1[0].payload.payload, "Hi, Alice!");
    
    // bobby replies to a message of alice

    const message_5_as_reply = {
      receiver: agent_pubkey_alice,
      payload: {
        type: "text",
        payload: "Sure! I would love to go out for coffee with you!",
      },
      replyTo: send_alice_3[1].id,
    };

    const reply_bobby = await sendMessage(message_5_as_reply)(bobby_cell);
    await delay(1000);

    // console.log(reply_bobby);
    t.deepEqual(reply_bobby[0].author, agent_pubkey_bobby);
    t.deepEqual(reply_bobby[0].receiver, agent_pubkey_alice);
    t.deepEqual(reply_bobby[0].payload.payload,"Sure! I would love to go out for coffee with you!");

    // // carly sends 2 messages to bobby
    // console.log("Carly sends three messages to Bobby");

    const send_carly_1 = await sendMessage(message_6)(carly_cell);
    await delay(1000);

    // console.log(send_carly_1);
    t.deepEqual(send_carly_1[0].author, agent_pubkey_carly);
    t.deepEqual(send_carly_1[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_carly_1[0].payload.payload, "Hi, Bobby!");


    const send_carly_2 = await sendMessage(message_7)(carly_cell);
    await delay(1000);


    console.log(send_carly_2);
    t.deepEqual(send_carly_2[0].author, agent_pubkey_carly);
    t.deepEqual(send_carly_2[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_carly_2[0].payload.payload,"I have an extra ticket to a movie later. Would you want to come with me?");

    // bobby sends a messages to carly
    // console.log("Bobby sends a message to Carly");

    const send_bobby_3 = await sendMessage(message_8)(bobby_cell);
    await delay(1000);

    // console.log(send_bobby_3);
    t.deepEqual(send_bobby_3[0].author, agent_pubkey_bobby);
    t.deepEqual(send_bobby_3[0].receiver, agent_pubkey_carly);
    t.deepEqual(send_bobby_3[0].payload.payload,"Hey, Carly. I'm sorry but I already made plans. maybe some other time?");

    // alice sends a message to bobby
    // console.log("Alice sends a message to bobby");
    
    const send_alice_4 = await sendMessage(message_9)(alice_cell);
    await delay(1000);

    // console.log(send_alice_4);
    t.deepEqual(send_alice_4[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_4[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_alice_4[0].payload.payload, "Great! I'll see you later!");

    // alice gets her latest messages
    const alice_latest_messages_1 = await getLatestMessages(1)(alice_cell);
    await delay(1000);

    // console.log("alice gets 1 latest message");
    for (var agent_key in alice_latest_messages_1[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_latest_messages_1[0][agent_key].length, 1);
      console.log("the messages with this agent are:");
      console.log(alice_latest_messages_1[0][agent_key]);
      for (var message_key in alice_latest_messages_1[1]) {
        console.log( "the message contents and receipts for this message hash are:");
        console.log(alice_latest_messages_1[1][message_key]);
        for (var receipt_key in alice_latest_messages_1[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_latest_messages_1[2][receipt_key]);
        }
      }
    }
    
    const alice_latest_messages_2 = await getLatestMessages(2)(alice_cell);
    await delay(1000);

    // console.log("alice gets 2 latest message");
    for (var agent_key in alice_latest_messages_2[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_latest_messages_2[0][agent_key].length, 2);
      console.log("the messages with this agent are:");
      console.log(alice_latest_messages_2[0][agent_key]);
      for (var message_key in alice_latest_messages_2[1]) {
        console.log("the message contents and receipts for this message hash are:");
        console.log(alice_latest_messages_2[1][message_key]);
        for (var receipt_key in alice_latest_messages_2[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_latest_messages_2[2][receipt_key]);
        }
      }
    }


    // bobby gets his latest message
    console.log("Bobby gets his latest messaages (size 3)"); //MANUEL : said three but then its sended one 

    
    const bobby_latest_messages_1 = await getLatestMessages(1)(bobby_cell);
    await delay(1000);



    // bobby gets the next batch of messages from alice
    // console.log("Bobby gets the next batch (size 1) of messages from Alice");

    const last_message_alice_hash_string = bobby_latest_messages_1[0][agent_pubkey_alice_string];
    const last_message_alice = bobby_latest_messages_1[1][last_message_alice_hash_string];
    const last_message_alice_receipt_hash_string = last_message_alice[1][0];
    const last_message_alice_receipt_id = bobby_latest_messages_1[2][last_message_alice_receipt_hash_string].id;

    const batch_filter_alice = {
      conversant: agent_pubkey_alice,
      batch_size: 3,
      payload_type: "Text",
      last_fetched_timestamp: last_message_alice[0].timeSent,
      last_fetched_message_id: last_message_alice_receipt_id,
    };

    const bobby_next_batch_1 = await getNextBatchMessages(batch_filter_alice)(bobby_cell);
    await delay(1000);

    for (var agent_key in bobby_next_batch_1[0]) {
      t.deepEqual(agent_key, agent_pubkey_alice_string);
      t.deepEqual(bobby_next_batch_1[0][agent_key].length, 3);
      console.log("the messages with this agent are:");
      console.log(bobby_next_batch_1[0][agent_key]);
      for (var message_key in bobby_next_batch_1[1]) {
        console.log( "the message contents and receipts for this message hash are:");
        console.log(bobby_next_batch_1[1][message_key]);
        for (var receipt_key in bobby_next_batch_1[2]) {
          console.log("the receipts for this message are:");
          console.log(bobby_next_batch_1[2][receipt_key]);
        }
      }
    }

    // bobby gets the next batch of messages from carly
    // console.log("Bobby gets the next batch (size 2) of messages from Carly");

    const last_message_carly_hash_string = bobby_latest_messages_1[0][agent_pubkey_carly_string];
    const last_message_carly = bobby_latest_messages_1[1][last_message_carly_hash_string];
    const last_message_carly_receipt_hash_string = last_message_carly[1][0];
    const last_message_carly_receipt_id = bobby_latest_messages_1[2][last_message_carly_receipt_hash_string].id;

    const batch_filter_carly = {
      conversant: agent_pubkey_carly,
      batch_size: 2,
      payload_type: "Text",
      last_fetched_timestamp: last_message_carly[0].timeSent,
      last_fetched_message_id: last_message_carly_receipt_id,
    };

    const bobby_next_batch_2 = await getNextBatchMessages(batch_filter_carly)(bobby_cell); 
    await delay(1000);

    for (var agent_key in bobby_next_batch_2[0]) {
      t.deepEqual(agent_key, agent_pubkey_carly_string);
      t.deepEqual(bobby_next_batch_2[0][agent_key].length, 2);
      console.log("the messages with this agent are:");
      console.log(bobby_next_batch_2[0][agent_key]);
      for (var message_key in bobby_next_batch_2[1]) {
        console.log("the message contents and receipts for this message hash are:");
        console.log(bobby_next_batch_2[1][message_key]);
        for (var receipt_key in bobby_next_batch_2[2]) {
          console.log("the receipts for this message are:");
          console.log(bobby_next_batch_2[2][receipt_key]);
        }
      }
    }

    // carly gets her messages with alice for today
    const today = [Math.floor(Date.now() / 1000), 0];

    // console.log("Carly gets her messages with Alice for today");
    const timestamp_filter_alice = {
      conversant: agent_pubkey_alice,
      date: today,
      payload_type: "Text",
    };

    const alice_today = await getMessagesByAgentByTimestamp(timestamp_filter_alice)(carly_cell);
    await delay(1000);
    
    for (var agent_key in alice_today[0]) {
      t.deepEqual(agent_key, agent_pubkey_alice_string);
      t.deepEqual(alice_today[0][agent_key].length, 0);
      console.log("the messages with this agent are:");
      console.log(alice_today[0][agent_key]);
      for (var message_key in alice_today[1]) {
        console.log("the message contents and receipts for this message hash are:");
        console.log(alice_today[1][message_key]);
        for (var receipt_key in alice_today[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_today[2][receipt_key]);
        }
      }
    }

// carly gets her messages with bobby for today
// console.log("Carly gets her messages with Bobby for today");
    
    const timestamp_filter_bobby = {
      conversant: agent_pubkey_bobby,
      date: today,
      payload_type: "Text",
    };

    const bobby_today = await  getMessagesByAgentByTimestamp(timestamp_filter_bobby)(carly_cell);
    await delay(1000);

    for (var agent_key in bobby_today[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(bobby_today[0][agent_key].length, 3);
      console.log("the messages with this agent are:");
      console.log(bobby_today[0][agent_key]);
      for (var message_key in bobby_today[1]) {
        console.log("the message contents and receipts for this message hash are:");
        console.log(bobby_today[1][message_key]);
        for (var receipt_key in bobby_today[2]) {
          console.log("the receipts for this message are:");
          console.log(bobby_today[2][receipt_key]);
        }
      }
    }

  // MANUEL: THIS TEST ITS FAILING  (theres a comment in the error line )

//---------------------------------------------------------------------------------------------------------------------------------------------------------------

    let text_1 = "The quick brown fox jumps over the lazy dog.";
    let file_text_1 = strToUtf8Bytes(text_1);
    const file_hash_1 = hash_file(Int8Array.from(file_text_1));

    // console.log(file_text_1);
    // console.log(file_hash_1);

    const message_10 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "file",
        file_name: "test file",
        file_size: 20,
        file_type: { type: "others" },
        file_hash: file_hash_1,
        bytes: file_text_1.toString(),
      },
      replyTo: null,
    };


    const alice_file_1 = await sendMessage(message_10)(alice_cell);
    await delay(1000)

    // console.log(alice_file_1);
    // console.log(alice_file_1[0].payload);

    const timestamp_filter_text_bobby = {
      conversant: agent_pubkey_bobby,
      date: today,
      payload_type: "Text",
    };

    const timestamp_filter_file_bobby = {
      conversant: agent_pubkey_bobby,
      date: today,
      payload_type: "File",
    };

    const timestamp_filter_all_bobby = {
      conversant: agent_pubkey_bobby,
      date: today,
      payload_type: "All",
    };


    const alice_today_text = await getMessagesByAgentByTimestamp(timestamp_filter_text_bobby)(bobby_cell);
    await delay(1000);

    // console.log("alice today texts with bobby");
    // console.log(alice_today_text);

    for (var agent_key in alice_today_text[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_today_text[0][agent_key].length, 6); // MANUEL:this its failing receive a 9 intead of 6 
      console.log("the messages with this agent are:");
      console.log(alice_today_text[0][agent_key]);
      for (var message_key in alice_today_text[1]) {
        console.log("the message contents and receipts for this message hash are:");
        console.log(alice_today_text[1][message_key]);
        console.log(alice_today_text[1][message_key][0]);
        for (var receipt_key in alice_today_text[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_today_text[2][receipt_key]);
        }
      }
    }



    //NO TESTED YET

    // const alice_today_file = await alice_cell.call(
    //   "p2pmessage",
    //   "get_messages_by_agent_by_timestamp",
    //   timestamp_filter_file_bobby
    // );
    // console.log("alice today files with bobby");
    // console.log(alice_today_file);

    // for (var agent_key in alice_today_file[0]) {
    //   t.deepEqual(agent_key, agent_pubkey_bobby_string);
    //   t.deepEqual(alice_today_file[0][agent_key].length, 1);
    //   console.log("the messages with this agent are:");
    //   console.log(alice_today_file[0][agent_key]);
    //   for (var message_key in alice_today_file[1]) {
    //     console.log(
    //       "the message contents and receipts for this message hash are:"
    //     );
    //     console.log(alice_today_file[1][message_key]);
    //     for (var receipt_key in alice_today_file[2]) {
    //       console.log("the receipts for this message are:");
    //       console.log(alice_today_file[2][receipt_key]);
    //     }
    //   }
    // }

    // const alice_today_all = await alice_cell.call(
    //   "p2pmessage",
    //   "get_messages_by_agent_by_timestamp",
    //   timestamp_filter_all_bobby
    // );
    // console.log("alice today all with bobby");
    // console.log(alice_today_all);

    // for (var agent_key in alice_today_all[0]) {
    //   t.deepEqual(agent_key, agent_pubkey_bobby_string);
    //   t.deepEqual(alice_today_all[0][agent_key].length, 7);
    //   console.log("the messages with this agent are:");
    //   console.log(alice_today_all[0][agent_key]);
    //   for (var message_key in alice_today_all[1]) {
    //     console.log(
    //       "the message contents and receipts for this message hash are:"
    //     );
    //     console.log(alice_today_all[1][message_key]);
    //     for (var receipt_key in alice_today_all[2]) {
    //       console.log("the receipts for this message are:");
    //       console.log(alice_today_all[2][receipt_key]);
    //     }
    //   }
    // }

    // const batch_filter_bobby_file = {
    //   conversant: agent_pubkey_bobby,
    //   batch_size: 2,
    //   payload_type: "File",
    //   payload_type_2: <File>{},
    //   last_fetched_timestamp: null,
    //   last_fetched_message_id: null,
    // };

    // const bobby_next_batch_file = await bobby_cell.call(
    //   "p2pmessage",
    //   "get_next_batch_messages",
    //   batch_filter_bobby_file
    // );
    // await delay(1000);

    // console.log(bobby_next_batch_file);
  });
  
  orchestrator.run();
};

export default messaging;
