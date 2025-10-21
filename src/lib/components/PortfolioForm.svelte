<script lang="ts">
  import { portfolioApi } from '$lib/api/portfolio';
  import type { Portfolio } from '$lib/types/portfolio';

  interface Props {
    portfolio?: Portfolio;
    onSave?: () => void;
    onCancel?: () => void;
  }

  let { portfolio, onSave, onCancel }: Props = $props();

  let name = $state(portfolio?.name || '');
  let description = $state(portfolio?.description || '');
  let saving = $state(false);
  let error = $state('');

  async function handleSubmit(event: Event) {
    event.preventDefault();
    
    // Validation
    if (!name.trim()) {
      error = 'Name is required';
      return;
    }

    try {
      saving = true;
      error = '';

      if (portfolio?.id) {
        // Update existing
        await portfolioApi.update({
          ...portfolio,
          name,
          description: description || undefined,
        });
      } else {
        // Create new
        await portfolioApi.create(name, description || undefined);
      }

      onSave?.();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to save portfolio';
      console.error('Error saving portfolio:', err);
    } finally {
      saving = false;
    }
  }

  function handleCancel() {
    onCancel?.();
  }
</script>

<form onsubmit={handleSubmit}>
  <div class="form-group">
    <label for="name">Name *</label>
    <input
      id="name"
      type="text"
      bind:value={name}
      placeholder="Enter portfolio name"
      disabled={saving}
      required
    />
  </div>

  <div class="form-group">
    <label for="description">Description</label>
    <textarea
      id="description"
      bind:value={description}
      placeholder="Enter portfolio description"
      disabled={saving}
      rows="4"
    ></textarea>
  </div>

  {#if error}
    <div class="error-message">{error}</div>
  {/if}

  <div class="button-group">
    <button type="submit" class="btn-primary" disabled={saving}>
      {saving ? 'Saving...' : 'Save'}
    </button>
    <button type="button" class="btn-secondary" onclick={handleCancel} disabled={saving}>
      Cancel
    </button>
  </div>
</form>

<style>
  form {
    max-width: 600px;
    margin: 0 auto;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #333;
  }

  input,
  textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
    font-family: inherit;
    box-sizing: border-box;
  }

  input:focus,
  textarea:focus {
    outline: none;
    border-color: #1976d2;
    box-shadow: 0 0 0 3px rgba(25, 118, 210, 0.1);
  }

  input:disabled,
  textarea:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  textarea {
    resize: vertical;
  }

  .error-message {
    padding: 0.75rem;
    margin-bottom: 1rem;
    background-color: #ffebee;
    color: #c62828;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .button-group {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }

  button {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background-color: #1976d2;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: #1565c0;
  }

  .btn-secondary {
    background-color: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: #d0d0d0;
  }
</style>
