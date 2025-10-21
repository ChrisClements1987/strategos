export interface Requirement {
  id?: number;
  client_id: number;
  name: string;
  description?: string;
  priority: 'low' | 'medium' | 'high' | 'critical';
  status: 'pending' | 'approved' | 'in_progress' | 'completed' | 'rejected';
  created_at?: string;
  updated_at?: string;
}
