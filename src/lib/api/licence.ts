import { invoke } from '@tauri-apps/api/core';
import type { Licence } from '$lib/types/licence';

export const licenceApi = {
  async create(portfolio_id: number, name: string, description?: string): Promise<number> {
    return await invoke('create_licence', { portfolio_id, name, description });
  },

  async getById(id: number): Promise<Licence> {
    return await invoke('get_licence', { id });
  },

  async getByPortfolio(portfolio_id: number): Promise<Licence[]> {
    return await invoke('get_licences_by_portfolio', { portfolio_id });
  },

  async addFeature(licence_id: number, feature_id: number): Promise<void> {
    return await invoke('add_feature_to_licence', { licence_id, feature_id });
  },

  async removeFeature(licence_id: number, feature_id: number): Promise<void> {
    return await invoke('remove_feature_from_licence', { licence_id, feature_id });
  },

  async getFeatures(licence_id: number): Promise<number[]> {
    return await invoke('get_licence_features', { licence_id });
  },

  async update(licence: Licence): Promise<void> {
    return await invoke('update_licence', { licence });
  },

  async delete(id: number): Promise<void> {
    return await invoke('delete_licence', { id });
  },
};
