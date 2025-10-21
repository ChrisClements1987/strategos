import { invoke } from '@tauri-apps/api/core';
import type { Portfolio } from '$lib/types/portfolio';

export const portfolioApi = {
  async create(name: string, description?: string): Promise<number> {
    return await invoke('create_portfolio', { name, description });
  },

  async getById(id: number): Promise<Portfolio> {
    return await invoke('get_portfolio', { id });
  },

  async getAll(): Promise<Portfolio[]> {
    return await invoke('get_all_portfolios');
  },

  async update(portfolio: Portfolio): Promise<void> {
    return await invoke('update_portfolio', { portfolio });
  },

  async delete(id: number): Promise<void> {
    return await invoke('delete_portfolio', { id });
  },
};
