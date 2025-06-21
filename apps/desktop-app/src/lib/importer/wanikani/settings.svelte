<script lang="ts">
    import { Input, Label, Helper, Button, Skeleton } from "flowbite-svelte";
    import { set, get, save } from "./store";
</script>

<form
    on:submit={async (event) => {
        event.preventDefault();
        console.log(event);
        const formData = new FormData(event.target as HTMLFormElement);
        console.log(formData.get("api-key"));

        await set("apiKey", formData.get("api-key") as string);
        await save()
    }}
>
    {#await get("apiKey")}
        <Skeleton size="sm" class="my-8" />
    {:then apiKey}
        <div>
            <Label for="api-key" class="mb-2">API Key</Label>
            <Input
                type="text"
                id="api-key"
                name="api-key"
                required
                clearable
                defaultValue={apiKey || ""}
            />
            <Helper class="text-sm">
                Your API key can be found <a
                    href="/"
                    class="text-primary-600 dark:text-primary-500 font-medium hover:underline"
                    >here</a
                >.
            </Helper>
        </div>
        <Button class="mt-4" type="submit">Submit</Button>
    {:catch error}
        <p>Error: {error}</p>
    {/await}
</form>
