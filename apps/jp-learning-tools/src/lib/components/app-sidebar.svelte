<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import HouseIcon from "@lucide/svelte/icons/house";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import LanguageIcon from "@lucide/svelte/icons/languages";
  import CameraIcon from "@lucide/svelte/icons/camera";

  const items = [
    { title: "Home", url: "/", icon: HouseIcon },
    { title: "Translate", url: "/translate", icon: LanguageIcon },
    {title: "Capture", url: "/ocr", icon: CameraIcon},
    { title: "Settings", url: "/settings", icon: SettingsIcon },
  ];
</script>

<Sidebar.Root class="w-min">
  <Sidebar.Header />
  <Sidebar.Content class="px-3">
    <Sidebar.Group />
    <Sidebar.GroupLabel>Application</Sidebar.GroupLabel>
    <Sidebar.GroupContent>
      <Sidebar.Menu>
        {#each items as item (item.title)}
          <Sidebar.MenuItem>
            <Sidebar.MenuButton>
              {#snippet child({ props })}
                <a href={item.url} {...props}>
                  <item.icon />
                  <span>{item.title}</span>
                </a>
              {/snippet}
            </Sidebar.MenuButton>
          </Sidebar.MenuItem>
        {/each}
      </Sidebar.Menu>
    </Sidebar.GroupContent>
    <Sidebar.Group />
  </Sidebar.Content>
  <Sidebar.Footer />
</Sidebar.Root>
