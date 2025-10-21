export interface Feature {
  id?: number;
  product_id: number;
  name: string;
  description?: string;
  status: 'planned' | 'in_progress' | 'completed' | 'cancelled';
  priority: 'low' | 'medium' | 'high' | 'critical';
  created_at?: string;
  updated_at?: string;
}
