<script lang="ts">
  import { Settings, User, Bell, Trash } from "@lucide/svelte";
  import {
    Avatar,
    Button,
    Dialog,
    DropdownMenu,
    Input,
    Select,
    Slider,
    Switch,
    Textarea,
    Tooltip,
  } from "@/components/ui";
  import { ChatCard } from "@/components/ChatCard";
  import { testChats } from "@/stories/test-data";

  let inputValue = $state("");
  let textareaValue = $state("");
  let dialogOpen = $state(false);
  let selectValue = $state("");
  let sliderValue = $state(50);
  let switchChecked = $state(false);

  let selectItems = $state([
    { value: "apple", label: "Apple" },
    { value: "banana", label: "Some Big Banana" },
    { value: "cherry", label: "Cherry" },
    { value: "orange", label: "Orange (disabled)", disabled: true },
  ]);
</script>

<svelte:head>
  <title>Storybook</title>
  <meta name="description" content="My storybook" />
</svelte:head>

<div class="min-h-screen p-8">
  <h1 class="mb-8 text-2xl font-bold">Storybook</h1>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Avatar</h2>
    <div class="flex max-w-72 gap-4">
      <Avatar
        src="https://avatars.githubusercontent.com/u/127850940?v=4"
        alt="User avatar"
        fallback="A"
      />
      <Avatar src={null} alt="User avatar" fallback="A" />
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Button</h2>
    <div class="flex flex-wrap gap-2">
      <Button variant="primary">Primary</Button>
      <Button variant="secondary">Secondary</Button>
      <Button variant="icon">
        <Settings class="h-5 w-5" />
      </Button>
      <Button variant="destructive">Destructive</Button>
      <Button variant="primary" disabled>Disabled</Button>
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Input</h2>
    <div class="flex max-w-72 flex-col gap-4">
      <Input bind:value={inputValue} placeholder="Enter text..." />
      <Input bind:value={inputValue} placeholder="Disabled" disabled />
      <p class="text-sm text-(--color-text-muted)">Value: {inputValue}</p>
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Textarea</h2>
    <div class="flex max-w-72 flex-col gap-4">
      <Textarea bind:value={textareaValue} placeholder="Enter description..." />
      <Textarea bind:value={textareaValue} placeholder="Disabled" disabled />
      <p class="text-sm text-(--color-text-muted)">Value: {textareaValue}</p>
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Dialog</h2>
    <Dialog.Root bind:open={dialogOpen}>
      <Dialog.Trigger>
        {#snippet child({ props })}
          <Button {...props}>Open Dialog</Button>
        {/snippet}
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content>
          <Dialog.Title>Dialog Title</Dialog.Title>
          <Dialog.Close />
          <div class="-mx-3 mb-3 max-h-[50vh] overflow-y-auto px-3 text-sm">
            {#each Array.from({ length: 10 })}
              <p class="mb-4 leading-normal">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor
                incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud
                exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute
                irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
                pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia
                deserunt mollit anim id est laborum.
              </p>
            {/each}
          </div>
          <div class="flex gap-2">
            <Dialog.Root>
              <Dialog.Trigger>
                {#snippet child({ props })}
                  <Button {...props}>Open nested dialog</Button>
                {/snippet}
              </Dialog.Trigger>
              <Dialog.Portal>
                <Dialog.Overlay />
                <Dialog.Content>
                  <Dialog.Title>Nested Dialog Title</Dialog.Title>
                  This is nested dialog content.

                  <div class="flex">
                    <Dialog.Root>
                      <Dialog.Trigger>
                        {#snippet child({ props })}
                          <Button {...props}>One more nested</Button>
                        {/snippet}
                      </Dialog.Trigger>
                      <Dialog.Portal>
                        <Dialog.Overlay />
                        <Dialog.Content>
                          <Dialog.Title>Nested Dialog Title</Dialog.Title>
                          This is nested dialog content.
                        </Dialog.Content>
                      </Dialog.Portal>
                    </Dialog.Root>
                  </div>
                </Dialog.Content>
              </Dialog.Portal>
            </Dialog.Root>

            <Button variant="secondary" onclick={() => (dialogOpen = false)}>Cancel</Button>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">DropdownMenu</h2>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <Tooltip>
            {#snippet trigger()}
              <Button {...props}>Open Menu</Button>
            {/snippet}
            {#snippet content()}
              Tooltip on Open Menu button
            {/snippet}
          </Tooltip>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content side="bottom" align="start">
          <DropdownMenu.Item>
            <User class="h-4 w-4" />
            Profile
          </DropdownMenu.Item>
          <DropdownMenu.Item>
            <Settings class="h-4 w-4" />
            Settings
          </DropdownMenu.Item>
          <DropdownMenu.Item>
            <Bell class="h-4 w-4" />
            Notifications
          </DropdownMenu.Item>
          <DropdownMenu.Item disabled>
            <Trash class="h-4 w-4" />
            Delete
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Tooltip</h2>
    <Tooltip>
      {#snippet trigger()}
        <Button variant="primary">Hover or focus me</Button>
      {/snippet}
      {#snippet content()}
        Tooltip on hover/focus
      {/snippet}
    </Tooltip>

    <Tooltip>
      {#snippet trigger()}
        <Button variant="secondary">Second button</Button>
      {/snippet}
      {#snippet content()}
        Tooltip on hover/focus
      {/snippet}
    </Tooltip>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Switch</h2>
    <div class="flex max-w-72 flex-col gap-4">
      <div class="flex items-center gap-3">
        <Switch bind:checked={switchChecked} />
        <span>Toggle me — {switchChecked ? "On" : "Off"}</span>
      </div>
      <div class="flex items-center gap-3">
        <Switch bind:checked={switchChecked} disabled />
      </div>
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Slider</h2>
    <div class="flex max-w-72 flex-col gap-4">
      <Slider bind:value={sliderValue} min={0} max={100} step={1} />
      <p class="text-sm text-(--color-text-muted)">Value: {sliderValue}</p>
      <Slider bind:value={sliderValue} min={0} max={100} step={1} disabled />
      <span class="text-sm text-(--color-text-muted)">Disabled</span>
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Select</h2>
    <div class="flex max-w-72 flex-col gap-4">
      <Select
        bind:value={selectValue}
        type="single"
        placeholder="Choose an option..."
        items={selectItems}
      />

      <Select
        bind:value={selectValue}
        type="single"
        placeholder="Choose an option..."
        disabled
        items={selectItems}
      />

      <p class="text-sm text-(--color-text-muted)">Value: {selectValue}</p>
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Chat list</h2>
    <div class="flex max-h-64 max-w-72 flex-col overflow-auto bg-(--gray-2)">
      {#each testChats as chat (chat.id)}
        <ChatCard {chat} currentUserId="user_self" />
      {/each}
    </div>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">Messages</h2>
    <Button href="/storybook/test-message-list">To test message list</Button>
  </section>

  <section class="mb-8">
    <h2 class="mb-4 text-lg font-semibold">MediaGrid</h2>
    <Button href="/storybook/media-grid">To media grid</Button>
  </section>
</div>
