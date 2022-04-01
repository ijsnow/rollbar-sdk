import { Config, Instance } from '@rollbar/wasm';

import { createInstance } from './index.node';

class Rollbar {
    private instance: Instance;

    constructor(config: Config) {
        this.instance = createInstance(config);
    }
}
