import { AppSignal } from "@holochain/conductor-api";
import { Orchestrator, Player } from "@holochain/tryorama";
import { Installables } from "../types";
import { extractPayloadFromSignal, delay } from "../utils";

let signalVal = [true, true, false];

const handleTypeSignal = (signal: AppSignal) => {
  return () => extractPayloadFromSignal(signal).isTyping;
};

function typing(typing_info) {
  return (conductor) => conductor.call("p2pmessage", "typing", typing_info);
}

function init() {
  return (conductor) => conductor.call("p2pmessage", "init", null);
}


const signals = async (conductorConfig, installation: Installables) => {
  let orchestrator = new Orchestrator();

  orchestrator.registerScenario("Typing signal test", async (s, t) => {
    const [alice, bob]: Player[] = await s.players([
      conductorConfig,
      conductorConfig,
    ]);

    const [[alice_happ]] = await alice.installAgentsHapps(installation.one);
    const [[bob_happ]] = await bob.installAgentsHapps(installation.one);

    const alice_cell = alice_happ.cells[0];
    const bob_cell = bob_happ.cells[0];

    const agent_pubkey_bob = bob_happ.agent;

    console.log("Nicko typing test");

    // bob.setSignalHandler((signal) => {
    //   console.log("Nicko 3 signal", signal);
    //   t.deepEqual(handleTypeSignal(signal)(), signalVal.shift());
    // });

    const result = await typing({
      agent: agent_pubkey_bob,
      isTyping: true,
    })(alice_cell);

    await delay(1000);

    console.log("Nicko 2 result", result);

    await typing({
      agent: agent_pubkey_bob,
      isTyping: false,
    })(alice_cell);

    await delay(1000);

    console.log("Nicko 4")

    // TATS: there is no tape assert here? Is this getting tested anywhere?
  });

  orchestrator.run();
};

export default signals;
