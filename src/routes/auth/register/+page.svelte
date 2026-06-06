<script lang="ts">
  import { goto } from "$app/navigation";
  import { authStore } from "@/lib/stores/auth-store.svelte";
  import { Button, Input, Loader } from "@/components/ui";
  import { resolve } from "$app/paths";

  let username = $state("");
  let email = $state("");
  let globalName = $state("");
  let password = $state("");
  let isLoading = $state(false);

  let formError = $state<string | null>(null);
  let fieldErrors = $state<Record<string, string[]>>({});

  async function handleSubmit(e: Event) {
    e.preventDefault();
    isLoading = true;
    formError = null;
    fieldErrors = {};

    const result = await authStore.register(username, email, globalName, password);
    isLoading = false;

    if (result) {
      goto(resolve("/chats"));
    } else if (authStore.error) {
      formError = authStore.error.message;
      fieldErrors = authStore.error.api_error?.errors ?? {};
    }
  }
</script>

<div class="flex h-screen items-center justify-center">
  <form onsubmit={handleSubmit} class="flex w-80 flex-col gap-4">
    <h1 class="text-center text-xl font-semibold">Create account</h1>

    {#if formError}
      <p class="text-center text-sm text-red-400">{formError}</p>
    {/if}

    <div class="flex flex-col gap-1">
      <Input bind:value={username} placeholder="Username" required />
      {#if fieldErrors.username}
        <p class="text-xs text-red-400">{fieldErrors.username[0]}</p>
      {/if}
    </div>

    <div class="flex flex-col gap-1">
      <Input bind:value={globalName} placeholder="Display name" required />
      {#if fieldErrors.global_name}
        <p class="text-xs text-red-400">{fieldErrors.global_name[0]}</p>
      {/if}
    </div>

    <div class="flex flex-col gap-1">
      <Input bind:value={email} type="email" placeholder="Email" required />
      {#if fieldErrors.email}
        <p class="text-xs text-red-400">{fieldErrors.email[0]}</p>
      {/if}
    </div>

    <div class="flex flex-col gap-1">
      <Input bind:value={password} type="password" placeholder="Password" required />
      {#if fieldErrors.password}
        <p class="text-xs text-red-400">{fieldErrors.password[0]}</p>
      {/if}
    </div>

    <Button type="submit" disabled={isLoading}>
      {#if isLoading}
        <Loader size="sm" />
      {:else}
        Register
      {/if}
    </Button>

    <p class="text-center text-sm text-(--color-text-muted)">
      Already have an account?
      <a href={resolve("/auth/login")} class="text-(--color-accent) hover:underline">Log in</a>
    </p>
  </form>
</div>
