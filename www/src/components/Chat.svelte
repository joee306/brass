<script>
    import Message from "./Message.svelte";
    import { onMount } from "svelte";
    export let current_chat;
    let msgs = [];
    let input_text = "";
    let send = () => {
        console.log("unsocket : ", input_text);
    };
    onMount(async () => {
        const { location } = window;
        let socket = new WebSocket(`ws://${location.host}/ws`);
        socket.onopen = () => {
            console.log("Connected");
        };
        socket.onmessage = (ev) => {
            msgs.push({
                pp: "image",
                name: "user",
                time: "time",
                text: ev.data,
            });
            msgs = msgs;
            return;
        };
        send = () => {
            if (input_text === "") return;
            socket.send(current_chat + "/" + input_text);
            console.log(current_chat + "/" + input_text);
            input_text = "";
        };
    });
    function onKey(k) {
        if (k.keyCode == 13) {
            send();
        }
    }
</script>

<div class="w-5/6 right-0 absolute overflow-y-hidden bottom-0 chat_h">
    <div class="h-scr overflow-y-scroll">
        {#each msgs as msg}
            <Message pp={msg} name={msg.name} time={msg.time} text={msg.text} />
        {/each}
    </div>
    <input
        class="m-4 p-1.5 overflow-hidden rounded-lg absolute bottom-2 dark:bg-slate-800 dark:text-zinc-50 outline-none focus:bg-gray-800"
        placeholder="type your next message ..."
        type="text"
        on:keydown={onKey}
        name=""
        bind:value={input_text}
    />
    <button
        class="bottom-2 absolute right-2 m-4 bg-transparent hover:bg-slate-100 p-1.5 rounded-lg text-red-800 hover:bg-transparent active:bg-gray-700"
        on:click={send}
    >
        ☭
    </button>
</div>
<svelte:window />

<style>
    input {
        width: calc(100% - 2rem);
    }
    .chat_h {
        height: calc(100vh - 3rem);
    }
</style>
