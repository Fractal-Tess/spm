import { writable } from 'svelte/store';
import { store } from './tauriStore';

type Config = {
  user: string;
  port: string;
  host: string;
  interface: string;
};

const createTunnelStore = async () => {
  const defaultConfig: Config = {
    user: '',
    port: '',
    host: '',
    interface: ''
  };
  const config = await store.get<Config>('config');

  const { subscribe, set, update } = writable<Config>(config || defaultConfig);

  return {
    subscribe,
    set: (ts: Config) => {
      store.set('config', ts);
      set(ts);
    },
    tunnel: () => {}
  };
};

export const ts = await createTunnelStore();
