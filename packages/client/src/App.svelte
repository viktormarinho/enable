<script lang=ts>
  import Router from 'svelte-spa-router';
  import Login from './lib/routes/Login.svelte';
  import Oi from './lib/routes/Oi.svelte';
  
  const routes = {
    '/': Login,
    '/zimbas': Oi
  };
</script>

<Router {routes} />
