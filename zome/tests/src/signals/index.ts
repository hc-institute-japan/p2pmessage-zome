import { AppSignal } from "@holochain/conductor-api";
import { FunctionType } from "../types";
import { extractPayloadFromSignal, delay } from "../utils";

let signalVal = [true, true, false];

const handleTypeSignal = (signal: AppSignal) => {
  return () => extractPayloadFromSignal(signal).isTyping;
};

const signals: FunctionType = (orchestrator, conductorConfig, installation) => {
  orchestrator.registerScenario("Typing signal test", async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);

    bob.setSignalHandler((signal) => {
      t.deepEqual(handleTypeSignal(signal)(), signalVal.shift());
    });

    const [[alice_happ]] = await alice.installAgentsHapps(installation);
    const [[bob_happ]] = await bob.installAgentsHapps(installation);

    const alice_cell = alice_happ.cells[0];

    const agent_pubkey_bob = bob_happ.agent;

    await alice_cell.call("p2pmessage", "typing", {
      agent: agent_pubkey_bob,
      isTyping: true,
    });
    await delay();

    await alice_cell.call("p2pmessage", "typing", {
      agent: agent_pubkey_bob,
      isTyping: true,
    });
    await delay();
    await alice_cell.call("p2pmessage", "typing", {
      agent: agent_pubkey_bob,
      isTyping: false,
    });
    await delay();
  });
};

export default signals;
