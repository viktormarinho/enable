<script lang="ts">
  import Router, { replace } from "svelte-spa-router";
  import wrap from "svelte-spa-router/wrap";
  import Login from "./lib/routes/Login.svelte";
  import { SvelteComponent } from "svelte";
  import { getNotificationsContext } from "svelte-notifications";

  type LazyLoadedPage = Promise<{
    default: typeof SvelteComponent;
  }>;

  const conditions = [
    async () => {
      const res = await fetch("/api/auth/me");
      return res.ok;
    },
  ];

  const routes = {
    "/": Login,
    "/admin": wrap({
      asyncComponent: () =>
        import("./lib/routes/Admin.svelte") as LazyLoadedPage,
      conditions,
    }),
    "/admin/project/:id": wrap({
      asyncComponent: () =>
        import("./lib/routes/Project.svelte") as LazyLoadedPage,
      conditions,
    }),
    "/admin/settings": wrap({
      asyncComponent: () => 
        import("./lib/routes/Settings.svelte") as LazyLoadedPage,
      conditions
    })
  };
  const { addNotification } = getNotificationsContext();

  function conditionsFailed(_: CustomEvent) {
    replace("/");
    addNotification({
      type: "error",
      position: "top-right",
      removeAfter: 3000,
      text: "You must be logged in to access this page. Redirecting to login...",
      id: "login-required",
    });
  }
</script>

<Router {routes} on:conditionsFailed={conditionsFailed} />
