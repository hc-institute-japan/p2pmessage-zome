import { Orchestrator } from '@holochain/tryorama'

const orchestrator = new Orchestrator()

require('./message')(orchestrator)

orchestrator.run()

