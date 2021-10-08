import {
  Config,
  NetworkType,
  InstallAgentsHapps,
  TransportConfigType,
} from "@holochain/tryorama";
import path from "path";
import messaging from "./messaging";
import getters from "./getters";
import receipts from "./receipts";
import signals from "./signals";
import pin from "./pin";
import playground from "./playground";

import { Installables } from "./types";

const network = {
  network_type: NetworkType.QuicBootstrap,
  transport_pool: [{ type: TransportConfigType.Quic }],
  bootstrap_service: "https://bootstrap-staging.holo.host/",
};

const conductorConfig = Config.gen({ network });

const p2pmessagedna = path.join(
  __dirname,
  "../../p2pmessage.workdir.dna/p2pmessage.dna"
);
const installAgent: InstallAgentsHapps = [[[p2pmessagedna]]];

const installables: Installables = {
  one: installAgent,
};

messaging(conductorConfig, installables);
getters(conductorConfig, installables);
receipts(conductorConfig, installables); // pass
// signals(conductorConfig, installables); // 0 tests
pin(conductorConfig, installables); // pass
// playground(conductorConfig, installables);
