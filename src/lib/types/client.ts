export interface Client {
  id?: number;
  portfolio_id: number;
  name: string;
  description?: string;
  client_type: 'persona' | 'enterprise' | 'organization';
  status: string;
  created_at?: string;
  updated_at?: string;
}
