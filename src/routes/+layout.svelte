<script lang="ts">
  import { page } from '$app/stores';
  
  const navItems = [
    { path: '/', label: 'Portfolios', icon: '📁' },
    { path: '/products', label: 'Products', icon: '📦' },
    { path: '/features', label: 'Features', icon: '⭐' },
    { path: '/licences', label: 'Licences', icon: '🎫' },
    { path: '/clients', label: 'Clients', icon: '👥' },
    { path: '/requirements', label: 'Requirements', icon: '📋' },
  ];
</script>

<div class="app-container">
  <nav class="sidebar">
    <div class="logo">
      <h1>Strategos</h1>
      <p>Portfolio Management</p>
    </div>
    
    <ul class="nav-list">
      {#each navItems as item}
        <li>
          <a 
            href={item.path} 
            class:active={$page.url.pathname === item.path}
          >
            <span class="icon">{item.icon}</span>
            <span class="label">{item.label}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <main class="content">
    <slot />
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background-color: #f5f5f5;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .app-container {
    display: flex;
    height: 100vh;
  }

  .sidebar {
    width: 250px;
    background-color: #1e293b;
    color: white;
    display: flex;
    flex-direction: column;
    padding: 1.5rem 0;
  }

  .logo {
    padding: 0 1.5rem 2rem;
    border-bottom: 1px solid #334155;
  }

  .logo h1 {
    margin: 0;
    font-size: 1.75rem;
    color: #60a5fa;
  }

  .logo p {
    margin: 0.25rem 0 0;
    font-size: 0.875rem;
    color: #94a3b8;
  }

  .nav-list {
    list-style: none;
    padding: 1rem 0;
    margin: 0;
    flex: 1;
  }

  .nav-list li {
    margin: 0;
  }

  .nav-list a {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.875rem 1.5rem;
    color: #cbd5e1;
    text-decoration: none;
    transition: all 0.2s;
  }

  .nav-list a:hover {
    background-color: #334155;
    color: white;
  }

  .nav-list a.active {
    background-color: #3b82f6;
    color: white;
    border-left: 4px solid #60a5fa;
    padding-left: calc(1.5rem - 4px);
  }

  .icon {
    font-size: 1.25rem;
  }

  .label {
    font-size: 0.9375rem;
    font-weight: 500;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }
</style>
