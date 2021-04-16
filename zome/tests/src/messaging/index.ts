import { Player } from "@holochain/tryorama";
import { Base64 } from "js-base64";
import { FunctionType } from "../types";
import { delay } from "../utils";
import path from "path";

const p2pmessagedna = path.join(__dirname, "../../../p2pmessage.dna.gz");

function serializeHash(hash) {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

const messaging: FunctionType = async (
  orchestrator,
  conductorConfig,
  installation
) => {
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

    console.log(agent_pubkey_alice);
    console.log(agent_pubkey_bobby);
    console.log(agent_pubkey_carly);

    const message_1 = {
      receiver: agent_pubkey_bobby,
      payload: { 
        type: "TEXT", 
        payload: { payload: "Hello, Bobby" } 
      },
      replyTo: null,
    };

    const message_2 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "TEXT",
        payload: { payload: "I was wondering if you were free today." }
      },
      replyTo: null,
    };

    const message_3 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "TEXT",
        payload: { payload: "Would you like to go out for coffee?" }
      },
      replyTo: null,
    };

    const message_4 = {
      receiver: agent_pubkey_alice,
      payload: { 
        type: "TEXT", 
        payload: { payload: "Hi, Alice!" } 
      },
      replyTo: null,
    };

    // reply in this sequence

    const message_6 = {
      receiver: agent_pubkey_bobby,
      payload: { 
        type: "TEXT", 
        payload: { payload: "Hi, Bobby!" }
      },
      replyTo: null,
    };

    const message_7 = {
      receiver: agent_pubkey_bobby,
      payload: {
        type: "TEXT",
        payload: { payload: "I have an extra ticket to a movie later. Would you want to come with me?" }
      },
      replyTo: null,
    };

    const message_8 = {
      receiver: agent_pubkey_carly,
      payload: {
        type: "TEXT",
        payload: { payload: "Hey, Carly. I'm sorry but I already made plans. maybe some other time?" }
      },
      replyTo: null,
    };

    const message_9 = {
      receiver: agent_pubkey_bobby,
      payload: { 
        type: "TEXT", 
        payload: { payload: "Great! I'll see you later!" }
      },
      replyTo: null,
    };

    // alice sends 3 messages to bobby
    console.log("Alice sends three messages to Bobby");
    const send_alice_1 = await alice_cell.call(
      "p2pmessage",
      "send_message",
      message_1
    );
    await delay(1000);
    console.log(send_alice_1);
    t.deepEqual(send_alice_1[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_1[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_alice_1[0].payload.payload.payload, "Hello, Bobby" );

    const send_alice_2 = await alice_cell.call(
      "p2pmessage",
      "send_message",
      message_2
    );
    await delay(1000);
    console.log(send_alice_2);
    t.deepEqual(send_alice_2[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_2[0].receiver, agent_pubkey_bobby);
    t.deepEqual(
      send_alice_2[0].payload.payload.payload,
      "I was wondering if you were free today."
    );

    const send_alice_3 = await alice_cell.call(
      "p2pmessage",
      "send_message",
      message_3
    );
    await delay(1000);
    console.log(send_alice_3);
    t.deepEqual(send_alice_3[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_3[0].receiver, agent_pubkey_bobby);
    t.deepEqual(
      send_alice_3[0].payload.payload.payload,
      "Would you like to go out for coffee?"
    );

    // bobby sends a message to alice
    console.log("Bobby sends a messages to Alice");
    const send_bobby_1 = await bobby_cell.call(
      "p2pmessage",
      "send_message",
      message_4
    );
    await delay(1000);
    console.log(send_bobby_1);
    t.deepEqual(send_bobby_1[0].author, agent_pubkey_bobby);
    t.deepEqual(send_bobby_1[0].receiver, agent_pubkey_alice);
    t.deepEqual(send_bobby_1[0].payload.payload.payload, "Hi, Alice!");

    console.log(send_alice_3[1].id);
    // bobby replies to a message of alice
    const message_5_as_reply = {
      receiver: agent_pubkey_alice,
      payload: {
        type: "TEXT",
        payload: { payload: "Sure! I would love to go out for coffee with you!" },
      },
      replyTo: send_alice_3[1].id,
    };

    const reply_bobby = await bobby_cell.call(
      "p2pmessage",
      "send_message",
      message_5_as_reply
    );
    await delay(1000);
    console.log("bob replies to a message of alice");
    console.log(reply_bobby);
    t.deepEqual(reply_bobby[0].author, agent_pubkey_bobby);
    t.deepEqual(reply_bobby[0].receiver, agent_pubkey_alice);
    t.deepEqual(
      reply_bobby[0].payload.payload.payload,
      "Sure! I would love to go out for coffee with you!"
    );

    // carly sends 2 messages to bobby
    console.log("Carly sends three messages to Bobby");
    const send_carly_1 = await carly_cell.call(
      "p2pmessage",
      "send_message",
      message_6
    );
    await delay(1000);
    console.log(send_carly_1);
    t.deepEqual(send_carly_1[0].author, agent_pubkey_carly);
    t.deepEqual(send_carly_1[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_carly_1[0].payload.payload.payload, "Hi, Bobby!");

    const send_carly_2 = await carly_cell.call(
      "p2pmessage",
      "send_message",
      message_7
    );
    await delay(1000);
    console.log(send_carly_2);
    t.deepEqual(send_carly_2[0].author, agent_pubkey_carly);
    t.deepEqual(send_carly_2[0].receiver, agent_pubkey_bobby);
    t.deepEqual(
      send_carly_2[0].payload.payload.payload,
      "I have an extra ticket to a movie later. Would you want to come with me?"
    );

    // bobby sends a messages to carly
    console.log("Bobby sends a message to Carly");
    const send_bobby_3 = await bobby_cell.call(
      "p2pmessage",
      "send_message",
      message_8
    );
    await delay(1000);
    console.log(send_bobby_3);
    t.deepEqual(send_bobby_3[0].author, agent_pubkey_bobby);
    t.deepEqual(send_bobby_3[0].receiver, agent_pubkey_carly);
    t.deepEqual(
      send_bobby_3[0].payload.payload.payload,
      "Hey, Carly. I'm sorry but I already made plans. maybe some other time?"
    );

    // alice sends a message to bobby
    console.log("Alice sends a message to bobby");
    const send_alice_4 = await alice_cell.call(
      "p2pmessage",
      "send_message",
      message_9
    );
    await delay(1000);
    console.log(send_alice_4);
    t.deepEqual(send_alice_4[0].author, agent_pubkey_alice);
    t.deepEqual(send_alice_4[0].receiver, agent_pubkey_bobby);
    t.deepEqual(send_alice_4[0].payload.payload.payload, "Great! I'll see you later!");

    // alice gets her latest messages
    const alice_latest_messages_1 = await alice_cell.call(
      "p2pmessage",
      "get_latest_messages",
      1
    );
    await delay(1000);
    console.log("alice gets 1 latest message");
    for (var agent_key in alice_latest_messages_1[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_latest_messages_1[0][agent_key].length, 1);
      console.log("the messages with this agent are:");
      console.log(alice_latest_messages_1[0][agent_key]);
      for (var message_key in alice_latest_messages_1[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(alice_latest_messages_1[1][message_key]);
        for (var receipt_key in alice_latest_messages_1[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_latest_messages_1[2][receipt_key]);
        }
      }
    }

    const alice_latest_messages_2 = await alice_cell.call(
      "p2pmessage",
      "get_latest_messages",
      2
    );
    await delay(1000);
    console.log("alice gets 2 latest message");
    for (var agent_key in alice_latest_messages_2[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_latest_messages_2[0][agent_key].length, 2);
      console.log("the messages with this agent are:");
      console.log(alice_latest_messages_2[0][agent_key]);
      for (var message_key in alice_latest_messages_2[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(alice_latest_messages_2[1][message_key]);
        for (var receipt_key in alice_latest_messages_2[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_latest_messages_2[2][receipt_key]);
        }
      }
    }

    // bobby gets his latest message
    console.log("Bobby gets his latest messaages (size 3)");
    const bobby_latest_messages_1 = await bobby_cell.call(
      "p2pmessage",
      "get_latest_messages",
      1
    );
    await delay(1000);
    console.log(bobby_latest_messages_1);

    // bobby gets the next batch of messages from alice
    console.log("Bobby gets the next batch (size 1) of messages from Alice");
    const last_message_alice_hash_string =
      bobby_latest_messages_1[0][agent_pubkey_alice_string];
    const last_message_alice =
      bobby_latest_messages_1[1][last_message_alice_hash_string];
    const last_message_alice_receipt_hash_string = last_message_alice[1][0];
    const last_message_alice_receipt_id =
      bobby_latest_messages_1[2][last_message_alice_receipt_hash_string].id;

    const batch_filter_alice = {
      conversant: agent_pubkey_alice,
      batch_size: 3,
      payload_type: "Text",
      last_fetched_timestamp: last_message_alice[0].timeSent,
      last_fetched_message_id: last_message_alice_receipt_id,
    };

    const bobby_next_batch_1 = await bobby_cell.call(
      "p2pmessage",
      "get_next_batch_messages",
      batch_filter_alice
    );
    await delay(1000);

    for (var agent_key in bobby_next_batch_1[0]) {
      t.deepEqual(agent_key, agent_pubkey_alice_string);
      t.deepEqual(bobby_next_batch_1[0][agent_key].length, 3);
      console.log("the messages with this agent are:");
      console.log(bobby_next_batch_1[0][agent_key]);
      for (var message_key in bobby_next_batch_1[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(bobby_next_batch_1[1][message_key]);
        for (var receipt_key in bobby_next_batch_1[2]) {
          console.log("the receipts for this message are:");
          console.log(bobby_next_batch_1[2][receipt_key]);
        }
      }
    }

    // bobby gets the next batch of messages from carly
    console.log("Bobby gets the next batch (size 2) of messages from Carly");
    const last_message_carly_hash_string =
      bobby_latest_messages_1[0][agent_pubkey_carly_string];
    const last_message_carly =
      bobby_latest_messages_1[1][last_message_carly_hash_string];
    const last_message_carly_receipt_hash_string = last_message_carly[1][0];
    const last_message_carly_receipt_id =
      bobby_latest_messages_1[2][last_message_carly_receipt_hash_string].id;

    const batch_filter_carly = {
      conversant: agent_pubkey_carly,
      batch_size: 2,
      payload_type: "Text",
      last_fetched_timestamp: last_message_carly[0].timeSent,
      last_fetched_message_id: last_message_carly_receipt_id,
    };

    const bobby_next_batch_2 = await bobby_cell.call(
      "p2pmessage",
      "get_next_batch_messages",
      batch_filter_carly
    );
    await delay(1000);

    for (var agent_key in bobby_next_batch_2[0]) {
      t.deepEqual(agent_key, agent_pubkey_carly_string);
      t.deepEqual(bobby_next_batch_2[0][agent_key].length, 2);
      console.log("the messages with this agent are:");
      console.log(bobby_next_batch_2[0][agent_key]);
      for (var message_key in bobby_next_batch_2[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(bobby_next_batch_2[1][message_key]);
        for (var receipt_key in bobby_next_batch_2[2]) {
          console.log("the receipts for this message are:");
          console.log(bobby_next_batch_2[2][receipt_key]);
        }
      }
    }

    // carly gets her messages with alice for today
    const today = [Math.floor(Date.now() / 1000), 0];

    console.log("Carly gets her messages with Alice for today");
    const timestamp_filter_alice = {
      conversant: agent_pubkey_alice,
      date: today,
      payload_type: "Text",
    };

    const alice_today = await carly_cell.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter_alice
    );
    await delay(1000);

    for (var agent_key in alice_today[0]) {
      t.deepEqual(agent_key, agent_pubkey_alice_string);
      t.deepEqual(alice_today[0][agent_key].length, 0);
      console.log("the messages with this agent are:");
      console.log(alice_today[0][agent_key]);
      for (var message_key in alice_today[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(alice_today[1][message_key]);
        for (var receipt_key in alice_today[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_today[2][receipt_key]);
        }
      }
    }

    // carly gets her messages with bobby for today
    console.log("Carly gets her messages with Bobby for today");
    const timestamp_filter_bobby = {
      conversant: agent_pubkey_bobby,
      date: today,
      payload_type: "Text",
    };

    const bobby_today = await carly_cell.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter_bobby
    );
    await delay(1000);

    for (var agent_key in bobby_today[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(bobby_today[0][agent_key].length, 3);
      console.log("the messages with this agent are:");
      console.log(bobby_today[0][agent_key]);
      for (var message_key in bobby_today[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(bobby_today[1][message_key]);
        for (var receipt_key in bobby_today[2]) {
          console.log("the receipts for this message are:");
          console.log(bobby_today[2][receipt_key]);
        }
      }
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

    function utf8_to_str(a) {
      for (var i = 0, s = ""; i < a.length; i++) {
        var h = a[i].toString(16);
        if (h.length < 2) h = "0" + h;
        s += "%" + h;
      }
      return decodeURIComponent(s);
    }

    function hash_file(input_text) {
      let file_hash = require("crypto")
        .createHash("sha256")
        .update(input_text)
        .digest("hex");
      return file_hash;
    }

    let text_1 = "The quick brown fox jumps over the lazy dog.";
    let file_text_1 = strToUtf8Bytes(text_1);
    const file_hash_1 = hash_file(Int8Array.from(file_text_1));

    console.log(file_text_1);
    console.log(file_hash_1);

    const message_10 = {
      receiver: agent_pubkey_bobby,
      payload: { 
        type: "FILE",
        payload: {
          metadata: {
            fileName: "test file",
            fileSize: 20,
            fileType: "OTHER",
          },
          fileType: { type: "IMAGE", payload: Int8Array.from(file_text_1) },
          fileBytes: file_text_1.toString(),
        }
      },
      replyTo: null,
    };

    const alice_file_1 = await alice_cell.call(
      "p2pmessage",
      "send_message",
      message_10
    );
    await delay(1000);
    console.log(alice_file_1);
    console.log(alice_file_1[0].payload);

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

    const alice_today_text = await alice_cell.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter_text_bobby
    );
    console.log("alice today texts with bobby");
    console.log(alice_today_text);

    for (var agent_key in alice_today_text[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_today_text[0][agent_key].length, 6);
      console.log("the messages with this agent are:");
      console.log(alice_today_text[0][agent_key]);
      for (var message_key in alice_today_text[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(alice_today_text[1][message_key]);
        console.log(alice_today_text[1][message_key][0]);
        for (var receipt_key in alice_today_text[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_today_text[2][receipt_key]);
        }
      }
    }

    const alice_today_file = await alice_cell.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter_file_bobby
    );
    console.log("alice today files with bobby");
    console.log(alice_today_file);

    for (var agent_key in alice_today_file[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_today_file[0][agent_key].length, 1);
      console.log("the messages with this agent are:");
      console.log(alice_today_file[0][agent_key]);
      for (var message_key in alice_today_file[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(alice_today_file[1][message_key]);
        for (var receipt_key in alice_today_file[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_today_file[2][receipt_key]);
        }
      }
    }

    const alice_today_all = await alice_cell.call(
      "p2pmessage",
      "get_messages_by_agent_by_timestamp",
      timestamp_filter_all_bobby
    );
    console.log("alice today all with bobby");
    console.log(alice_today_all);

    for (var agent_key in alice_today_all[0]) {
      t.deepEqual(agent_key, agent_pubkey_bobby_string);
      t.deepEqual(alice_today_all[0][agent_key].length, 7);
      console.log("the messages with this agent are:");
      console.log(alice_today_all[0][agent_key]);
      for (var message_key in alice_today_all[1]) {
        console.log(
          "the message contents and receipts for this message hash are:"
        );
        console.log(alice_today_all[1][message_key]);
        for (var receipt_key in alice_today_all[2]) {
          console.log("the receipts for this message are:");
          console.log(alice_today_all[2][receipt_key]);
        }
      }
    }

    const batch_filter_bobby_file = {
      conversant: agent_pubkey_bobby,
      batch_size: 2,
      payload_type: "File",
      payload_type_2: <File>{},
      last_fetched_timestamp: null,
      last_fetched_message_id: null,
    };

    const bobby_next_batch_file = await bobby_cell.call(
      "p2pmessage",
      "get_next_batch_messages",
      batch_filter_bobby_file
    );
    await delay(1000);

    console.log(bobby_next_batch_file);
  });
};

export default messaging;
