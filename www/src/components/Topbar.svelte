<script>
    export let current_chat;
    if (current_chat === undefined) {
        current_chat = "";
    }
</script>

<div class="">
    <span
        class="text-2xl p-2 text-center dark:bg-slate-950 dark:text-zinc-50 top-0 absolute right-0 w-5/6 h-12 z-10"
        >{current_chat}</span
    >
</div>
