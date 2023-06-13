<script lang="ts">
    import Chat from "./Chat.svelte";
    import Topbar from "./Topbar.svelte";
    import { onMount } from "svelte";
    let logged = true;
    let username: string = "";
    let contacts: { username: string; public_key: string }[] = [];
    let current_chat;
    const change_chat = (i: number) => {
        console.log(i);
        current_chat = contacts[i].username;
        current_chat = current_chat;
    };
    const userdata = async (iterartion: number) => {
        const { location } = window;
        const res: any = await fetch(`http://${location.host}/api/getdata`, {
            method: "GET",
        });
        const text = await res;
        if (res.ok) {
            return text.json();
        } else {
            if (iterartion == 3) {
                logged = false;
            } else {
                userdata(iterartion + 1);
            }
            return;
        }
    };
    onMount(async () => {
        userdata(0).then((value) => {
            console.log(value);
            username = value.username;
            contacts = value.contacts;
            if (contacts.length !== 0) {
                current_chat = contacts[0].username;
            } else {
                current_chat = "You don't have Friends.";
            }
        });
    });
</script>

{#if logged}
    <Topbar {current_chat} />
    <Chat {current_chat} />
    <div class="h-screen bg-slate-950 w-2/12">
        <h1
            class="text-zinc-50 font-bold p-1 px-3 text-lg bg-slate-800 mx-2 top-2 rounded-b-2xl hover:underline"
        >
            Friends
        </h1>
        {#each contacts as contact, i}
            <p>
                <button
                    class="px-3 py-2 mx-4 my-1 hover:bg-slate-800 rounded-2xl dark:text-zinc-50"
                    on:click={() => {
                        change_chat(i);
                    }}
                >
                    {contact.username}
                </button>
            </p>
        {/each}
    </div>
{:else}
    <div
        class="h-screen bg-slate-950 w-screen flex items-center justify-center"
    >
        <div class="grid">
            <h1 class="text-zinc-50 font-bold text-lg">
                You are not logged in
            </h1>
            <button
                class="font-bold text-zinc-50 bg-slate-800 p-1 rounded-2xl"
                on:click={() => {
                    window.location.replace("/auth");
                }}>Go to Login page</button
            >
            >
        </div>
    </div>
{/if}
