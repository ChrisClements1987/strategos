import { invoke } from '@tauri-apps/api/core';
import type { Client } from '$lib/types/client';

export const clientApi = {
  async create(portfolio_id: number, name: string, description?: string): Promise<number> {
    return await invoke('create_client', { portfolio_id, name, description });
  },

  async getById(id: number): Promise<Client> {
    return await invoke('get_client', { id });
  },

  async getByPortfolio(portfolio_id: number): Promise<Client[]> {
    return await invoke('get_clients_by_portfolio', { portfolio_id });
  },

  async update(client: Client): Promise<void> {
    return await invoke('update_client', { client });
  },

  async delete(id: number): Promise<void> {
    return await invoke('delete_client', { id });
  },
};
