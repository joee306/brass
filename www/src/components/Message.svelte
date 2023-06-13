<script>
    export let pp;
    export let name;
    export let text;
    export let time;
</script>

<div class="rounded-lg bg-slate-800 m-4 p-2">
    <img
        class="w-6 m-0.5 aspect-square rounded-full float-left"
        src={pp}
        alt=""
    />
    <span class="font-semibold mx-2 my-4 text-white text-lg">{name} : </span>
    <span class="font-light mx-2 text-white right-4 absolute">{time}</span>
    <p class="pt-1.5 mx-2 text-white font-normal">{text}</p>
</div>
